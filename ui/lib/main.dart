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
      title: 'Sessio Test',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Sessio Test'),
    );
  }
}
