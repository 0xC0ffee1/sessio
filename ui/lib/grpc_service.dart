import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';
import 'package:network_info_plus/network_info_plus.dart';
import 'package:sessio_ui/main.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';
import 'package:path_provider/path_provider.dart';

class GrpcService {
  late ClientIPCClient _client;

  Future<void> init() async {
    this._client = await _createClientIPCClient();
  }

  Future<SessionMap> getActiveSessions() async {
    final response = await _client.getActiveSessions(SessionRequest());
    return response;
  }

  Future<void> deleteSessionSave(String id) async {
    final user_data = await _client.getSaveData(GetSaveDataRequest());
    user_data.savedSessions.remove(id);
    await _client.saveUserData(user_data);
  }

  Future<ClientIPCClient> _createClientIPCClient() async {
    final ClientChannel channel;
    Directory appDir = await getApplicationSupportDirectory();
    if (Platform.isAndroid) {
      //Waiting for the tokio runtime to start
      await Future.delayed(Duration(seconds: 1));
    }

    if (Platform.isLinux || Platform.isMacOS || Platform.isAndroid) {
      String unixPath = Platform.isAndroid
          ? appDir.path + "/sessio.socket"
          : "/tmp/sessio.socket";
      final InternetAddress host =
          InternetAddress(unixPath, type: InternetAddressType.unix);
      channel = ClientChannel(
        host,
        options: const ChannelOptions(
          credentials: ChannelCredentials.insecure(),
        ),
      );
    } else {
      channel = ClientChannel(
        'localhost',
        port: 53051, // Replace with your actual server port
        options: const ChannelOptions(
          credentials: ChannelCredentials.insecure(),
        ),
      );
    }
    final client = ClientIPCClient(channel);

    final Directory appDocumentsDir = await getApplicationDocumentsDirectory();

    bool success = false;
    while (!success) {
      try {
        final res = await client
            .initClient(InitData(dataFolderPath: appDocumentsDir.path));
        success = true;
        ScaffoldMessenger.of(navigatorKey.currentContext!).showSnackBar(
          SnackBar(
            content: Text('Connected to daemon'),
            behavior: SnackBarBehavior.floating,
            backgroundColor:
                Theme.of(navigatorKey.currentContext!).colorScheme.primary,
            duration: Duration(seconds: 1),
          ),
        );
      } catch (error) {
        if (error is GrpcError) {
          // Show AlertDialog
          final result = await showDialog(
            context: navigatorKey.currentContext!,
            builder: (BuildContext context) {
              return AlertDialog(
                title: Row(
                  children: [
                    Icon(Icons.error, color: Colors.red),
                    SizedBox(width: 10),
                    Text('Could not connect to daemon'),
                  ],
                ),
                content: Text('${error.message}'),
                actions: [
                  TextButton(
                    onPressed: () {
                      Navigator.of(context).pop('retry');
                    },
                    child: Text('Retry'),
                  ),
                  TextButton(
                    onPressed: () {
                      Navigator.of(context).pop('quit');
                    },
                    child: Text('Quit'),
                  ),
                ],
              );
            },
          );

          if (result == 'quit') {
            break;
          }
        } else {
          print('Unknown error: $error');
          break;
        }
      }
    }

    return ClientIPCClient(channel);
  }

  ClientIPCClient get client {
    return _client;
  }

  Future<String> getIpv6() async {
    for (var interface in await NetworkInterface.list()) {
      for (var addr in interface.addresses) {
        print(addr);
        if (addr.type == InternetAddressType.IPv6 &&
            addr.isLinkLocal == false) {
          return addr.address;
        }
      }
    }
    return "";
  }

  Future<NewSessionResponse> _newSession(SessionData data) async {
    Settings settings = await client.getSettings(SettingsRequest());
    var wifiIPv6 = await getIpv6();
    wifiIPv6 = wifiIPv6.split("%")[
        0]; //For some reason android adds this to even non link-local addresses?

    NewConnectionResponse connectionResponse =
        await client.newConnection(NewConnectionRequest()
          ..coordinatorUrl = settings.coordinatorUrl
          ..targetId = data.deviceId
          ..ownIpv6 = wifiIPv6);

    NewSessionResponse sessionResponse =
        await client.newSession(NewSessionRequest()
          ..privateKey = "keys/id_ed25519"
          ..knownHostsPath = "known_hosts"
          ..sessionData = data);

    return sessionResponse;
  }

  Future<SftpBrowser> connectSFTP(
      String clientId, String username, String? sessionId) async {
    final t = DateTime.now().millisecondsSinceEpoch;
    NewSessionResponse sessionResponse = await _newSession(SessionData(
        sessionId: sessionId,
        deviceId: clientId,
        username: username,
        sftp: SessionData_SFTPSession()));

    final res = await client
        .openSftpChannel(SessionData(sessionId: sessionResponse.sessionId));
    final browser = SftpBrowser(client, sessionResponse.sessionId);
    await browser.refreshFileList();
    return browser;
  }

  void connectLPF(
      String clientId,
      String username,
      String hostLocal,
      int portLocal,
      String hostRemote,
      int portRemote,
      String? sessionId) async {
    final data = SessionData(
        sessionId: sessionId,
        deviceId: clientId,
        username: username,
        lpf: SessionData_LPFSession(
            localHost: hostLocal,
            localPort: portLocal,
            remoteHost: hostRemote,
            remotePort: portRemote));

    final t = DateTime.now().millisecondsSinceEpoch;
    NewSessionResponse sessionResponse = await _newSession(data);
    data.sessionId = sessionResponse.sessionId;

    await client.localPortForward(data);
  }

  void connectPTY(String clientId, String username, SessioTerminalState state,
      String? sessionId) async {
    final streamController = state.streamController;
    var t = DateTime.now().millisecondsSinceEpoch;
    NewSessionResponse sessionResponse = await _newSession(SessionData(
        sessionId: sessionId,
        username: username,
        deviceId: clientId,
        pty: SessionData_PTYSession()));

    state.terminal
        .write("Connected! Session ID is: ${sessionResponse.sessionId} \r\n");

    final responseStream = client.openChannel(streamController.stream);

    streamController.add(Msg()
      ..channelInit =
          (Msg_ChannelInit()..sessionId = sessionResponse.sessionId));

    streamController.add(Msg()
      ..ptyRequest = (Msg_PtyRequest()
        ..colWidth = state.terminal.viewWidth
        ..rowHeight = state.terminal.viewHeight));

    // Handle terminal output
    state.terminal.onOutput = (data) {
      // Add the data to the stream
      streamController
          .add(Msg()..data = (Msg_Data()..payload = data.codeUnits));
      // Write the data to the terminal
      //state.terminal.write(data);
    };

    state.terminal.onResize = (w, h, pw, ph) {
      streamController.add(Msg()
        ..ptyResize = (Msg_PtyResize()
          ..colWidth = w
          ..rowHeight = h));
    };

    bool pinged = false;

    //state.terminal.buffer.clear();
    //state.terminal.buffer.setCursor(0, 0);

    streamController.add(Msg()..shellRequest = (Msg_ShellRequest()));
    // Handle responses
    await for (var response in responseStream) {
      // Handle the response
      if (response.hasData()) {
        if (!pinged) {
          state.terminal.write(
              "Took to connect: ${DateTime.now().millisecondsSinceEpoch - t}ms.\r\n");

          pinged = true;
        }
        String data = utf8.decode(response.data.payload, allowMalformed: true);
        //state.terminal.write("\b \b");
        try {
          state.terminal.write(data);
        } catch (e) {}
      }
    }
  }
}
