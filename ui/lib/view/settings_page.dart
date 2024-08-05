import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';

class SettingsPage extends StatefulWidget {
  @override
  _SettingsPageState createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  late Future<Settings> _settingsFuture;
  final TextEditingController _urlController = TextEditingController();
  final TextEditingController _deviceIdController = TextEditingController();
  late GrpcService _grpcService;
  String publicKey = '';

  @override
  void initState() {
    super.initState();
    _grpcService = Provider.of<GrpcService>(context, listen: false);
    _settingsFuture = _grpcService.client.getSettings(SettingsRequest());

    _loadPublicKey();
    _loadInitialSettings();
  }

  void _loadPublicKey() async {
    publicKey = (await _grpcService.client.getPublicKey(GetKeyRequest())).key;
  }

  void _loadInitialSettings() async {
    try {
      final settings = await _settingsFuture;
      _urlController.text = settings.coordinatorUrl;
      _deviceIdController.text = settings.deviceId;
    } catch (e) {}
  }

  Future<void> _saveSettings() async {
    final newSettings = Settings(
      coordinatorUrl: _urlController.text,
      deviceId: _deviceIdController.text,
    );

    try {
      await Provider.of<GrpcService>(context, listen: false)
          .client
          .saveSettings(newSettings);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Settings saved successfully'),
          behavior: SnackBarBehavior.floating,
          backgroundColor: Theme.of(context).colorScheme.primary,
          duration: Duration(seconds: 1),
        ),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Failed to save settings: $e'),
          behavior: SnackBarBehavior.floating,
          backgroundColor: Theme.of(context).colorScheme.error,
          duration: Duration(seconds: 1),
        ),
      );
    }
  }

  void _generateKeyPair() async {
    // Replace this with your actual key generation logic
    await _grpcService.client.genKeys(GenKeysRequest());
    final newKey =
        (await _grpcService.client.getPublicKey(GetKeyRequest())).key;
    setState(() {
      publicKey = newKey;
    });
  }

  void _copyToClipboard() {
    Clipboard.setData(ClipboardData(text: publicKey));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Public key copied to clipboard')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Settings'),
      ),
      body: FutureBuilder<Settings>(
        future: _settingsFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError) {
            return Center(child: Text('Error: ${snapshot.error}'));
          } else if (!snapshot.hasData) {
            return Center(child: Text('No data found.'));
          } else {
            return ListView(
              padding: EdgeInsets.all(16.0), // Add padding for better layout
              children: [
                ListTile(
                  leading: Icon(Icons.public),
                  title: Text('Coordinator URL'),
                  subtitle: TextField(
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                      hintText: 'Enter your coordinator URL',
                    ),
                    controller: _urlController,
                  ),
                ),
                SizedBox(height: 16.0), // Add space between fields
                ListTile(
                  leading: Icon(Icons.perm_identity),
                  title: Text('Device ID'),
                  subtitle: TextField(
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                      hintText: 'Enter the ID of this device',
                    ),
                    controller: _deviceIdController,
                  ),
                ),
                ListTile(
                  leading: Icon(Icons.key),
                  title: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text('Public Key'),
                      SizedBox(height: 8.0),
                      TextField(
                        controller: TextEditingController(text: publicKey),
                        readOnly: true,
                        decoration: InputDecoration(
                          border: OutlineInputBorder(),
                          hintText: 'No keys generated',
                          suffixIcon: publicKey.isNotEmpty
                              ? IconButton(
                                  icon: Icon(Icons.copy),
                                  onPressed: _copyToClipboard,
                                )
                              : null,
                        ),
                      ),
                      SizedBox(height: 8.0),
                      ElevatedButton(
                        onPressed: _generateKeyPair,
                        child: Text('Generate new pair'),
                      ),
                    ],
                  ),
                )
              ],
            );
          }
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _saveSettings,
        backgroundColor: Colors.pink,
        child: Icon(Icons.save),
      ),
    );
  }

  @override
  void dispose() {
    _urlController.dispose();
    _deviceIdController.dispose();
    super.dispose();
  }
}
