import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_background_service/flutter_background_service.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

import 'package:material_symbols_icons/symbols.dart';
import 'package:path_provider/path_provider.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';
import 'package:sessio_ui/view/mobile_keyboard.dart';
import 'package:sessio_ui/view/settings_page.dart';
import 'package:sessio_ui/view/sftp_browser.dart';
import 'package:sessio_ui/view/terminal_session_view.dart';
import 'package:workmanager/workmanager.dart';
import 'package:xterm/xterm.dart';

import '../main.dart';

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
  bool _showVirtualKeyboard = false;
  final PageController _pageController = PageController();

  Map<String, List<Widget>> sessionTree = {};
  Map<String, bool> deviceStatus = {};

  late Future<void> serviceFuture;

  @override
  void initState() {
    super.initState();
    serviceFuture = _loadSessions();
    initAndroid();
  }

  void initAndroid() {
    if (!Platform.isAndroid) return;
    checkPerms();
    initializeService();
  }

  Future<void> _loadSessions() async {
    await Provider.of<GrpcService>(context, listen: false).init();
    final sessionMap = await Provider.of<GrpcService>(context, listen: false)
        .getActiveSessions();

    for (var entry in sessionMap.map.entries) {
      var sessionData = entry.value;
      var clientId = sessionData.deviceId;

      // Update device status
      deviceStatus[clientId] = sessionMap.parents[clientId]?.connected ?? false;

      if (sessionData.hasPty()) {
        await _addNewSession(
            clientId, sessionData.username, 'PTY', sessionData.sessionId);
      } else if (sessionData.hasSftp()) {
        await _addNewSession(
            clientId, sessionData.username, 'SFTP', sessionData.sessionId);
      } else if (sessionData.hasLpf()) {
        var lpf = sessionData.lpf;
        await _addLocalPFSession(
            clientId,
            sessionData.username,
            lpf.localHost,
            lpf.localPort,
            lpf.remoteHost,
            lpf.remotePort,
            sessionData.sessionId);
      }
    }
  }

  Future<void> initializeService() async {
    if (await FlutterBackgroundService().isRunning()) return;

    final service = FlutterBackgroundService();
    const notificationId = 888;
    const notificationChannelId = 'sessio_grpc_service';

    const AndroidNotificationChannel channel = AndroidNotificationChannel(
      notificationChannelId, // id
      'sessio_grpc_service', // title
      description:
          'This channel is used for important notifications.', // description
      importance: Importance.low, // importance must be at low or higher level
    );

    final FlutterLocalNotificationsPlugin flutterLocalNotificationsPlugin =
        FlutterLocalNotificationsPlugin();

    await flutterLocalNotificationsPlugin
        .resolvePlatformSpecificImplementation<
            AndroidFlutterLocalNotificationsPlugin>()
        ?.createNotificationChannel(channel);

    await service.configure(
        androidConfiguration: AndroidConfiguration(
          // this will be executed when app is in foreground or background in separated isolate
          onStart: onStart,

          // auto start service
          autoStart: true,
          isForegroundMode: true,

          notificationChannelId:
              notificationChannelId, // this must match with notification channel you created above.
          initialNotificationTitle: 'Sessio',
          initialNotificationContent: 'Sessio is running',
          foregroundServiceNotificationId: notificationId,
        ),
        iosConfiguration: IosConfiguration());
  }

  void checkPerms() async {
    if (!await Permission.manageExternalStorage.isGranted) {
      await Permission.manageExternalStorage.request();
    }
    if (!await Permission.notification.isGranted &&
        !await Permission.notification.isPermanentlyDenied) {
      await Permission.notification.request();
    }
  }

  Future<void> _showClientIdDialog() async {
    TextEditingController clientIdController = TextEditingController();
    TextEditingController usernameController = TextEditingController();
    TextEditingController localHostPortController = TextEditingController();
    TextEditingController remoteHostPortController = TextEditingController();
    String sessionType = "PTY"; // Default session type

    await showDialog(
      context: context,
      builder: (context) {
        return StatefulBuilder(
          builder: (context, setState) {
            return AlertDialog(
              title: Text('Enter Device ID'),
              content: SingleChildScrollView(
                child: Padding(
                  padding: EdgeInsets.only(
                      bottom: MediaQuery.of(context).viewInsets.bottom),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      _buildTextField(usernameController, 'Username'),
                      SizedBox(height: 10),
                      _buildTextField(clientIdController, 'Device ID'),
                      SizedBox(height: 20),
                      Text(
                        'Select Session Type',
                        style: TextStyle(fontWeight: FontWeight.bold),
                      ),
                      SizedBox(height: 10),
                      Wrap(
                        spacing: 10.0,
                        children: [
                          _buildFilterChip(
                            context,
                            setState,
                            'PTY',
                            sessionType,
                            Icons.terminal,
                            () => setState(() => sessionType = 'PTY'),
                          ),
                          _buildFilterChip(
                            context,
                            setState,
                            'SFTP',
                            sessionType,
                            Icons.folder_open,
                            () => setState(() => sessionType = 'SFTP'),
                          ),
                          _buildFilterChip(
                            context,
                            setState,
                            'L-PF',
                            sessionType,
                            Symbols.valve,
                            () => setState(() => sessionType = 'L-PF'),
                          ),
                        ],
                      ),
                      if (sessionType == 'L-PF') ...[
                        SizedBox(height: 10),
                        _buildTextField(
                            localHostPortController, 'Local Host:Port'),
                        SizedBox(height: 10),
                        _buildTextField(
                            remoteHostPortController, 'Remote Host:Port'),
                      ],
                    ],
                  ),
                ),
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
                    if (sessionType == 'L-PF') {
                      var localHostPort =
                          localHostPortController.text.split(':');
                      var remoteHostPort =
                          remoteHostPortController.text.split(':');
                      _addLocalPFSession(
                          clientIdController.text,
                          usernameController.text,
                          localHostPort[0],
                          int.parse(localHostPort[1]),
                          remoteHostPort[0],
                          int.parse(remoteHostPort[1]),
                          null);
                    } else {
                      _addNewSession(clientIdController.text,
                          usernameController.text, sessionType, null);
                    }
                  },
                ),
              ],
            );
          },
        );
      },
    );
  }

  Widget _buildTextField(TextEditingController controller, String hintText) {
    return TextField(
      controller: controller,
      decoration: InputDecoration(
        border: OutlineInputBorder(),
        hintText: hintText,
      ),
    );
  }

  Widget _buildFilterChip(
      BuildContext context,
      StateSetter setState,
      String label,
      String selectedType,
      IconData icon,
      VoidCallback onSelected) {
    bool isSelected = selectedType == label;
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: FilterChip(
        avatar: Icon(icon, color: isSelected ? Colors.white : Colors.black),
        label: Container(
          width: 50,
          child: Center(child: Text(label)),
        ),
        selected: isSelected,
        onSelected: (selected) => onSelected(),
        selectedColor: Colors.pink,
        backgroundColor: Colors.grey[200],
        labelStyle: TextStyle(color: isSelected ? Colors.white : Colors.black),
        showCheckmark: false,
      ),
    );
  }

  Future<void> _addLocalPFSession(
      String clientId,
      String username,
      String hostLocal,
      int portLocal,
      String hostRemote,
      int portRemote,
      String? sessionId) async {
    Provider.of<GrpcService>(context, listen: false).connectLPF(clientId,
        username, hostLocal, portLocal, hostRemote, portRemote, sessionId);

    if (!sessionTree.containsKey(clientId)) {
      sessionTree[clientId] = [];
    }
    sessionTree[clientId]!.add(
      Row(
        children: [
          Icon(Symbols.valve),
          SizedBox(width: 8),
          Text("L-PF"),
        ],
      ),
    );

    setState(() {
      sessionViews.add(Scaffold(
        appBar: AppBar(title: Text("Local port forward")),
      ));
    });
  }

  Future<void> _addNewSession(
      String clientId, String username, String type, String? sessionId) async {
    if (!sessionTree.containsKey(clientId)) {
      sessionTree[clientId] = [];
    }
    int currentIndex = sessionTree[clientId]!.length;
    sessionTree[clientId]!.add(
      Row(
        children: [
          Icon(type == "PTY" ? Icons.terminal : Icons.folder_open),
          SizedBox(width: 8),
          Text(type),
          Spacer(),
          IconButton(onPressed: () {
            if(sessionId != null){
              Provider.of<GrpcService>(context, listen: false).deleteSessionSave(sessionId);
            }
            setState(() {
              sessionTree[clientId]!.removeAt(currentIndex);
              if(sessionTree[clientId]!.isEmpty) {
                sessionTree.remove(clientId);
              }
            });

          }, icon: Icon(Icons.delete))
        ],
      ),
    );

    if (type == "PTY") {
      final sessionState = SessioTerminalState();
      final keyboard = VirtualKeyboard(defaultInputHandler);
      sessionState.terminal.inputHandler = keyboard;
      setState(() {
        sessionViews.add(TerminalSessionView(
          terminalState: sessionState,
          keyboard: keyboard,
        ));

        WidgetsBinding.instance.addPostFrameCallback((_) {
          Provider.of<GrpcService>(context, listen: false)
              .connectPTY(clientId, username, sessionState, sessionId);
        });
      });
    } else if (type == "SFTP") {
      SftpBrowser browser =
          await Provider.of<GrpcService>(context, listen: false)
              .connectSFTP(clientId, username, sessionId);
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
    final theme = Theme.of(context);
    return Row(children: [
      NavigationRail(
        backgroundColor: theme.colorScheme.surfaceBright,
        indicatorColor: const Color.fromARGB(50, 233, 30, 99),
        selectedIndex: _selectedRailIndex <= 1 ? _selectedRailIndex : 0,
        minWidth: 80,
        onDestinationSelected: (int index) {
          setState(() {
            _selectedRailIndex = index;
            _updateCurrentPageIndex(index);
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
      ),
      _buildMioNavigationDrawer()
    ]);
  }

  Widget _buildSessionListView() {
    int offset = 0;
    return ListView(
      padding: EdgeInsets.zero, // Remove any padding
      children: [
        Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(children: [
            SizedBox(height: 20),
            TextButton(
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
            )
          ]),
        ),
        ...sessionTree.keys.map((parent) {
          final tile = ExpansionTile(
            shape: Border(),
            title: Row(children: [
              _buildConnStatus(),
              SizedBox(width: 8),
              Text(
                parent,
              )
            ]),
            children: sessionTree[parent]!.asMap().entries.map((entry) {
              int index = entry.key + offset;
              Widget session = entry.value;
              return Padding(
                padding: const EdgeInsets.only(left: 16.0),
                child: ListTile(
                  title: session,
                  selected: _selectedSessionIndex - 2 == index,
                  selectedColor: Colors.pink,
                  onTap: () {
                    setState(() {
                      _selectedSessionIndex =
                          index + 2; // Ensure session indices start from 2
                      print("Setting index to ${_selectedSessionIndex}");
                    });
                  },
                ),
              );
            }).toList(),
          );
          offset += sessionTree[parent]!.length;
          return tile;
        }).toList(),
      ],
    );
  }

  Widget _buildMioNavigationDrawer() {
    final theme = Theme.of(context);
    return Row(
      children: [
        Stack(
          children: [
            AnimatedContainer(
              duration: Duration(milliseconds: 200),
              curve: Curves.easeIn,
              width: _isDrawerOpen ? 200 : 0,
              color: theme.colorScheme
                  .surfaceContainerHigh, // Ensure background color matches
              child: ClipRect(
                child: Align(
                    alignment: Alignment.topLeft,
                    widthFactor: _isDrawerOpen ? 1.0 : 0.0,
                    child: _buildSessionListView()),
              ),
            ),
          ],
        ),
        VerticalDivider(thickness: 1, width: 1),
      ],
    );
  }

  void _updateCurrentPageIndex(int index) {
    if (index != 0 && _isDrawerOpen) {
      _isDrawerOpen = !_isDrawerOpen;
    } else if (index == 0) {
      _isDrawerOpen = !_isDrawerOpen;
    }
  }

  Widget _buildSessionPage() {
    return Row(children: [
      Expanded(
          child: _selectedSessionIndex > 1
              ? sessionViews[_selectedSessionIndex - 2]
              : Center(child: Text('No sessions yet!'))),
    ]);
  }

  Widget _buildSessionPageSmall() {
    return Scaffold(
      appBar: AppBar(
        title: Text("Sessions"),
        leading: Builder(
          builder: (context) => IconButton(
            icon: Icon(Icons.menu),
            onPressed: () {
              // Open the drawer using the context provided by Builder
              Scaffold.of(context).openDrawer();
            },
          ),
        ),
      ),
      drawer: Drawer(child: _buildSessionListView()),
      body: _selectedSessionIndex > 1
          ? sessionViews[_selectedSessionIndex - 2]
          : Center(child: Text('No sessions yet!')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: serviceFuture,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return Scaffold(
            body: Center(child: CircularProgressIndicator()),
          );
        } else if (snapshot.hasError) {
          return Scaffold(
            body: Center(child: Text('Error: ${snapshot.error}')),
          );
        } else {
          return Scaffold(
            body: LayoutBuilder(
              builder: (context, constraints) {
                if (constraints.maxWidth > 600) {
                  // Larger screens
                  return Row(
                    children: [
                      _buildMioLeftNavRail(),
                      Expanded(
                        child: AnimatedSwitcher(
                          duration: Duration(milliseconds: 100),
                          transitionBuilder:
                              (Widget child, Animation<double> animation) {
                            return FadeTransition(
                              opacity: animation,
                              child: child,
                            );
                          },
                          child: _selectedRailIndex == 0
                              ? _buildSessionPage()
                              : SettingsPage(),
                        ),
                      ),
                    ],
                  );
                } else {
                  // Smaller screens
                  return Column(
                    children: [
                      Expanded(
                        child: PageView(
                          controller: _pageController,
                          onPageChanged: (index) {
                            setState(() {
                              _selectedRailIndex = index;
                            });
                          },
                          children: [_buildSessionPageSmall(), SettingsPage()],
                        ),
                      ),
                      BottomNavigationBar(
                        currentIndex: _selectedRailIndex,
                        onTap: (int index) {
                          setState(() {
                            _selectedRailIndex = index;
                            _pageController.animateToPage(index,
                                duration: Duration(milliseconds: 200),
                                curve: Curves.easeIn);
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
      },
    );
  }
}
