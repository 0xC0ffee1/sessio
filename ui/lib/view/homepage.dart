import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/view/sftp_browser.dart';
import 'package:xterm/xterm.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  List<Widget> sessions = [];
  List<Widget> sessionViews = [];
  int _selectedRailIndex = 0;
  int _selectedSessionIndex = 0;
  bool _isDrawerOpen = true; // New state variable to track drawer state

  Map<String, List<Widget>> sessionTree = {};

  Future<void> _showClientIdDialog() async {
    TextEditingController clientIdController = TextEditingController();
    String sessionType = "PTY"; // Default session type

    await showDialog(
      context: context,
      builder: (context) {
        return StatefulBuilder(
          builder: (context, setState) {
            return AlertDialog(
              title: Text('Enter Device ID'),
              content: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextField(
                    controller: clientIdController,
                    decoration: InputDecoration(
                        border: OutlineInputBorder(), hintText: "Device ID"),
                  ),
                  SizedBox(height: 20),
                  Text(
                    'Select Session Type',
                    style: TextStyle(fontWeight: FontWeight.bold),
                  ),
                  SizedBox(height: 10),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: [
                      FilterChip(
                        avatar: Icon(Icons.terminal,
                            color: sessionType == 'PTY'
                                ? Colors.white
                                : Colors.black),
                        label: Container(
                          width: 50, // Ensures consistent width
                          child: Center(
                            child: Text('PTY'),
                          ),
                        ),
                        selected: sessionType == 'PTY',
                        onSelected: (selected) {
                          setState(() {
                            sessionType = 'PTY';
                          });
                        },
                        selectedColor: Colors.pink,
                        backgroundColor: Colors.grey[200],
                        labelStyle: TextStyle(
                          color: sessionType == 'PTY'
                              ? Colors.white
                              : Colors.black,
                        ),
                        showCheckmark: false,
                      ),
                      FilterChip(
                        avatar: Icon(Icons.folder_open,
                            color: sessionType == 'SFTP'
                                ? Colors.white
                                : Colors.black),
                        label: Container(
                          width: 50, // Ensures consistent width
                          child: Center(
                            child: Text('SFTP'),
                          ),
                        ),
                        selected: sessionType == 'SFTP',
                        onSelected: (selected) {
                          setState(() {
                            sessionType = 'SFTP';
                          });
                        },
                        selectedColor: Colors.pink,
                        backgroundColor: Colors.grey[200],
                        labelStyle: TextStyle(
                          color: sessionType == 'SFTP'
                              ? Colors.white
                              : Colors.black,
                        ),
                        showCheckmark: false,
                      ),
                    ],
                  ),
                ],
              ),
              actions: [
                TextButton(
                  child: Text('Cancel'),
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                ),
                TextButton(
                  child: Text('Connect'),
                  onPressed: () {
                    Navigator.of(context).pop();
                    _addNewSession(clientIdController.text, sessionType);
                  },
                ),
              ],
            );
          },
        );
      },
    );
  }

  Future<void> _addNewSession(String clientId, String type) async {


    if (!sessionTree.containsKey(clientId)) {
      sessionTree[clientId] = [];
    }
    sessionTree[clientId]!.add(
      Row(
        children: [
          Icon(type == "PTY" ? Icons.terminal : Icons.folder_open),
          SizedBox(width: 8),
          Text(type),
          //add del button here: Icon(Icons.delete)
        ],
      ),
    );

    if (type == "PTY") {
      final sessionState = SessioTerminalState();
      final terminal = sessionState.terminal;
      final terminalController = sessionState.terminalController;
      setState(() {
        sessionViews.add(
          Center(
            child: TerminalView(
              terminal,
              controller: terminalController,
              autofocus: true,
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
            ),
          ),
        );
        WidgetsBinding.instance.addPostFrameCallback((_) {
          
          Provider.of<GrpcService>(context, listen: false)
              .connectPTY(clientId, sessionState);
        });
      });
    } else if (type == "SFTP") {
      SftpBrowser browser =
          await Provider.of<GrpcService>(context, listen: false)
              .connectSFTP(clientId);
      setState(() {
        sessionViews.add(
          Center(child: FileBrowserView(browser: browser)),
        );
      });
    }
  }

  Widget _buildConnStatus() {
    return Tooltip(
        message: 'Online',
        child: Icon(Icons.circle_outlined, color: Colors.green, size: 15));
  }

  Widget _buildMioLeftNavRail() {
    return NavigationRail(
      backgroundColor: Color.fromARGB(255, 45, 45, 45),
      indicatorColor: const Color.fromARGB(50, 233, 30, 99),
      selectedIndex: _selectedRailIndex <= 1 ? _selectedRailIndex : 0,
      minWidth: 80,
      onDestinationSelected: (int index) {
        setState(() {
          _selectedRailIndex = index;
        });
      },
      labelType: NavigationRailLabelType.all,
      destinations: [
        NavigationRailDestination(
          icon: Icon(Icons.home_outlined),
          selectedIcon: Icon(Icons.home_filled),
          label: Text('Sessions'),
        ),
        NavigationRailDestination(
          icon: Icon(Icons.settings_outlined),
          selectedIcon: Icon(Icons.settings),
          label: Text('Settings'),
        ),
      ],
    );
  }

  Widget _buildMioNavigationDrawer() {
    return Row(
      children: [
        AnimatedContainer(
          duration: Duration(milliseconds: 125),
          width: _isDrawerOpen ? 200 : 0,
          child: Material(
            color: Color.fromARGB(
                255, 40, 40, 40), // Ensure background color matches
            child: ListView(
              padding: EdgeInsets.zero, // Remove any padding
              children: [
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: TextButton(
                    onPressed: () async {
                      await _showClientIdDialog();
                    },
                    child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Icon(Icons.add),
                          SizedBox(width: 10),
                          Text('New Session')
                        ]),
                  ),
                ),
                ...sessionTree.keys.map((parent) {
                  return ExpansionTile(
                    shape: Border(),
                    title: Row(children: [
                      _buildConnStatus(),
                      SizedBox(width: 8),
                      Text(
                        parent,
                        style: TextStyle(color: Colors.white),
                      )
                    ]),
                    children: sessionTree[parent]!.asMap().entries.map((entry) {
                      int index = entry.key;
                      Widget session = entry.value;
                      return Padding(
                        padding: const EdgeInsets.only(left: 16.0),
                        child: ListTile(
                          title: session,
                          selected: _selectedSessionIndex - 2 == index,
                          selectedColor: Colors.pink,
                          onTap: () {
                            setState(() {
                              _selectedSessionIndex = index +
                                  2; // Ensure session indices start from 2
                            });
                          },
                        ),
                      );
                    }).toList(),
                  );
                }).toList(),
              ],
            ),
          ),
        ),
        VerticalDivider(thickness: 1, width: 1),
        _buildDrawerToggleButton()
      ],
    );
  }

  Widget _buildDrawerToggleButton() {
    return Transform.translate(
      offset: Offset(-25, 0), // Move the icon 25px to the left
      child: TextButton(
        onPressed: () {
          setState(() {
            _isDrawerOpen = !_isDrawerOpen;
          });
        },
        child: Icon(
          _isDrawerOpen ? Icons.arrow_back_ios : Icons.arrow_forward_ios,
          color: Colors.white,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: LayoutBuilder(
        builder: (context, constraints) {
          if (constraints.maxWidth > 600) {
            // Larger screens
            return Stack(
              children: [
                Row(
                  children: [
                    _buildMioLeftNavRail(),
                    VerticalDivider(thickness: 1, width: 1),
                    if (_selectedRailIndex == 0) ...[
                      _buildMioNavigationDrawer(),
                      Expanded(
                        child: _selectedSessionIndex > 1
                            ? sessionViews[_selectedSessionIndex - 2]
                            : Center(
                                child: Text('No sessions yet!'),
                              ),
                      ),
                    ],
                  ],
                ),
              ],
            );
          } else {
            // Smaller screens
            return Column(
              children: [
                Expanded(
                  child: _selectedSessionIndex > 1
                      ? sessionViews[_selectedSessionIndex - 2]
                      : Center(
                          child: Text('Settings'),
                        ),
                ),
                BottomNavigationBar(
                  currentIndex:
                      _selectedRailIndex <= 1 ? _selectedRailIndex : 0,
                  onTap: (int index) {
                    setState(() {
                      _selectedRailIndex = index;
                    });
                  },
                  items: [
                    BottomNavigationBarItem(
                      icon: Icon(Icons.home),
                      label: 'Sessions',
                    ),
                    BottomNavigationBarItem(
                      icon: Icon(Icons.settings),
                      label: 'Settings',
                    ),
                  ],
                ),
              ],
            );
          }
        },
      ),
    );
  }
}
