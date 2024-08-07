import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sessio_ui/main.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';
import 'package:sessio_ui/view/mobile_keyboard.dart';
import 'package:xterm/xterm.dart';

class TerminalSessionView extends StatefulWidget {
  final SessioTerminalState terminalState;
  final dynamic keyboard;

  TerminalSessionView({required this.terminalState, required this.keyboard});

  @override
  _TerminalSessionViewState createState() => _TerminalSessionViewState();
}

class _TerminalSessionViewState extends State<TerminalSessionView> {
  bool _showVirtualKeyboard = false;
  final FocusNode focusNode = FocusNode();

  void _toggleVirtualKeyboard() {
    setState(() {
      _showVirtualKeyboard = !_showVirtualKeyboard;
    });
  }

  void _handleKeyEvent(KeyEvent event) {
    if (!Platform.isAndroid) return;
  }

  //@TODO Maybe wrap some invisible text field over the terminalview to capture and forward keystrokes on android
  @override
  Widget build(BuildContext context) {
    final terminal = widget.terminalState.terminal;
    final terminalController = widget.terminalState.terminalController;
    final theme = Theme.of(context);

    final terminalView = TerminalView(
      terminal,
      controller: terminalController,
      autofocus: true,
      theme: theme.brightness == Brightness.light
          ? getLightTheme()
          : TerminalThemes.defaultTheme,
      backgroundOpacity: 0.0,
      onSecondaryTapDown: (details, offset) async {
        final selection = terminalController.selection;
        if (selection != null) {
          final text = terminal.buffer.getText(selection);
          terminalController.clearSelection();
          await Clipboard.setData(ClipboardData(text: text));
        } else {
          final data = await Clipboard.getData('text/plain');
          final text = data?.text;
          if (text != null) {
            terminal.paste(text);
          }
        }
      },
    );

    return Scaffold(
      body: LayoutBuilder(
        builder: (context, constraints) {
          if (constraints.maxWidth < 600) {
            return Column(
              children: [
                Expanded(child: terminalView),
              ],
            );
          } else {
            return Column(
              children: [
                Expanded(
                  child: Center(
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Container(
                        padding: const EdgeInsets.all(16.0),
                        decoration: BoxDecoration(
                          color: theme.colorScheme.surfaceContainer,
                          borderRadius: BorderRadius.circular(12.0),
                        ),
                        child: terminalView,
                      ),
                    ),
                  ),
                ),
              ],
            );
          }
        },
      ),
      floatingActionButton: LayoutBuilder(
        builder: (context, constraints) {
          if (constraints.maxWidth < 600 ||
              Platform.isAndroid ||
              Platform.isIOS) {
            return ExpandableFab(
              distance: 112,
              keyboard: widget.keyboard,
              children: [
                ActionButton(
                  onPressed: () => setState(
                      () => widget.keyboard.ctrl = !widget.keyboard.ctrl),
                  icon:
                      const Text('Ctrl', style: TextStyle(color: Colors.black)),
                  active: widget.keyboard.ctrl,
                ),
                ActionButton(
                  onPressed: () => setState(
                      () => widget.keyboard.alt = !widget.keyboard.alt),
                  icon:
                      const Text('Alt', style: TextStyle(color: Colors.black)),
                  active: widget.keyboard.alt,
                ),
                ActionButton(
                  onPressed: () => setState(
                      () => widget.keyboard.shift = !widget.keyboard.shift),
                  icon: const Text('Shift',
                      style: TextStyle(color: Colors.black)),
                  active: widget.keyboard.shift,
                ),
              ],
            );
          } else {
            return Container(); // Render nothing if the screen is larger than 600px
          }
        },
      ),
    );
  }

  TerminalTheme getLightTheme() {
    return TerminalTheme(
        cursor: Colors.black,
        selection: Colors.black,
        foreground: Colors.black,
        background: Colors.white,
        black: Colors.black,
        white: Colors.white,
        red: Colors.red,
        green: Colors.green,
        yellow: Color.fromARGB(255, 155, 135, 12),
        blue: Colors.blue,
        magenta: Colors.purple,
        cyan: Colors.cyan,
        brightBlack: Colors.black26,
        brightRed: Colors.redAccent,
        brightGreen: Colors.greenAccent,
        brightYellow: Color.fromARGB(255, 236, 183, 83),
        brightBlue: Colors.blueAccent,
        brightMagenta: Colors.purpleAccent,
        brightCyan: Colors.cyanAccent,
        brightWhite: Colors.white30,
        searchHitBackground: Colors.white,
        searchHitBackgroundCurrent: Colors.black,
        searchHitForeground: Colors.black);
  }
}
