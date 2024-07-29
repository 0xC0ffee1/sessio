import 'package:flutter/material.dart';
import 'package:xterm/xterm.dart';

class SessioTerminalState with ChangeNotifier {
  final Terminal terminal = Terminal(maxLines: 10000);
  final TerminalController terminalController = TerminalController();

  void addOutput(String output) {
    terminal.write(output);
    notifyListeners();
  }
}
