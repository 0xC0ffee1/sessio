import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:grpc/grpc.dart';
import 'package:injectable/injectable.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';
import 'package:xterm/xterm.dart';

@singleton
class GrpcService {
  final ClientIPCClient client;

  GrpcService() : client = _createClientIPCClient();

  static ClientIPCClient _createClientIPCClient() {
    final ClientChannel channel;
    if (Platform.isLinux || Platform.isMacOS) {
      final InternetAddress host =
          InternetAddress('/tmp/sessio.socket', type: InternetAddressType.unix);
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
    return ClientIPCClient(channel);
  }

  Future<NewSessionResponse> _newSession(String clientId) async {
    NewConnectionResponse connectionResponse =
        await client.newConnection(NewConnectionRequest()
          ..coordinatorUrl = "quic://157.90.127.19:2223"
          ..targetId = clientId);

    NewSessionResponse sessionResponse =
        await client.newSession(NewSessionRequest()
          ..connectionId = connectionResponse.connectionId
          ..privateKey = "keys/id_ed25519"
          ..knownHostsPath = "known_hosts"
          ..username = "test-ses");

    return sessionResponse;
  }

  Future<SftpBrowser> connectSFTP(String clientId) async {
    final t = DateTime.now().millisecondsSinceEpoch;
    NewSessionResponse sessionResponse = await _newSession(clientId);

    final res = await client
        .openSftpChannel(SftpRequest(sessionId: sessionResponse.sessionId));
    final browser = SftpBrowser(client, sessionResponse.sessionId);
    await browser.refreshFileList();
    return browser;
  }

  bool isSpecialCharacter(String data) {
    // List of special characters to check for
    const specialChars = [
      '\r',
      '\n',
      '\x03',
      '\x04'
    ]; // Enter, Ctrl+C, Ctrl+D, etc.

    // Check if the data contains any special characters
    return specialChars.any((char) => data.contains(char));
  }

  void connectPTY(String clientId, SessioTerminalState state) async {
    var t = DateTime.now().millisecondsSinceEpoch;
    NewSessionResponse sessionResponse = await _newSession(clientId);

    final streamController = StreamController<Msg>();
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
        String data = utf8.decode(response.data.payload);
        //state.terminal.write("\b \b");
        state.terminal.write(data);
      }
    }
  }
}
