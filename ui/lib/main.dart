import 'package:flutter/material.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/view/homepage.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(
    MultiProvider(
      providers: [
        Provider<GrpcService>(create: (_) => GrpcService()),
        ChangeNotifierProvider(create: (context) => SessioTerminalState()),
      ],
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Sessio',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.pink),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.pink,
        // Additional dark theme settings if needed
      ),
      themeMode: ThemeMode.system,
      home: const MyHomePage(title: 'Sessio Test'),
    );
  }
}
