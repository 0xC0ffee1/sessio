import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';
import 'package:xterm/xterm.dart';

class SessioTerminalState with ChangeNotifier {
  final Terminal terminal = Terminal(maxLines: 10000);
  final TerminalController terminalController = TerminalController();
  final StreamController<Msg> streamController = StreamController();
}
