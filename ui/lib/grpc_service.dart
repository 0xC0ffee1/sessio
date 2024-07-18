import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:grpc/grpc.dart';
import 'package:injectable/injectable.dart';
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
          InternetAddress('tmp/test.socket', type: InternetAddressType.unix);
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

  void connect(String clientId, SessioTerminalState state) async {
    final t = DateTime.now().millisecondsSinceEpoch;
    NewConnectionResponse connectionResponse =
        await client.newConnection(NewConnectionRequest()
          ..coordinatorUrl = "quic://127.0.0.1:2223"
          ..targetId = clientId);

    NewSessionResponse sessionResponse =
        await client.newSession(NewSessionRequest()
          ..connectionId = connectionResponse.connectionId
          ..privateKey = "keys/id_ed25519"
          ..knownHostsPath = "known_hosts"
          ..username = "test-ses");

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

    state.terminal.onOutput = (data) {
      streamController
          .add(Msg()..data = (Msg_Data()..payload = data.codeUnits));
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
              "Took ${DateTime.now().millisecondsSinceEpoch - t}ms to connect.\r\n");
          pinged = true;
        }
        state.terminal.write(utf8.decode(response.data.payload));
      }
    }
  }
}
