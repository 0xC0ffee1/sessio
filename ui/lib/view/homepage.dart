import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:xterm/xterm.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> with TickerProviderStateMixin {
  final List<Tab> myTabs = <Tab>[];
  final List<Widget> myTabViews = <Widget>[];
  TabController? _tabController;

  @override
  void initState() {
    super.initState();
    myTabs.add(_buildAddTab());
    myTabViews.add(Container()); // Placeholder for the "+ Add" tab
    _tabController = TabController(vsync: this, length: myTabs.length);
  }

  Tab _buildAddTab() {
    return Tab(icon: Icon(Icons.add));
  }

  Future<void> _showClientIdDialog() async {
    TextEditingController clientIdController = TextEditingController();
    await showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text('Enter Client ID'),
          content: TextField(
            controller: clientIdController,
            decoration: InputDecoration(hintText: "Client ID"),
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
                _addNewTab(clientIdController.text);
              },
            ),
          ],
        );
      },
    );
  }

  void _addNewTab(String clientId) {
    final sessionState = SessioTerminalState();
    final terminal = sessionState.terminal;
    final terminalController = sessionState.terminalController;

    setState(() {
      myTabs.insert(
          myTabs.length - 1,
          Tab(
            child: Row(
              children: [
                Icon(Icons.terminal),
                SizedBox(width: 8),
                Text(clientId),
                //add del button here: Icon(Icons.delete)
              ],
            ),
          ));

      myTabViews.insert(
        myTabViews.length - 1,
        Center(
          child: TerminalView(
            terminal,
            controller: terminalController,
            autofocus: true,
            backgroundOpacity: 1.0,
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

      _tabController!.dispose();
      _tabController = TabController(
        vsync: this,
        length: myTabs.length,
      );
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      final service = Provider.of<GrpcService>(context, listen: false);
      service.connect(clientId, sessionState);
      _tabController!.animateTo(myTabs.length - 2);
      //setState(() {});
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        bottom: TabBar(
          controller: _tabController,
          isScrollable: true,
          tabs: myTabs,
          onTap: (index) {
            if (index == myTabs.length - 1) {
              _showClientIdDialog();
            }
          },
        ),
        actions: [
          PopupMenuButton<String>(
            onSelected: (String value) {
              if (value == 'Settings') {}
            },
            itemBuilder: (BuildContext context) {
              return {'Settings'}.map((String choice) {
                return PopupMenuItem<String>(
                  value: choice,
                  child: Text(choice),
                );
              }).toList();
            },
          ),
        ],
      ),
      body: TabBarView(
        controller: _tabController,
        children: myTabViews,
      ),
    );
  }

  @override
  void dispose() {
    _tabController?.dispose();
    super.dispose();
  }
}
