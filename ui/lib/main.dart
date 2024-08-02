import 'package:flutter/material.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/view/homepage.dart';
import 'package:provider/provider.dart';

final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
final GlobalKey<ScaffoldMessengerState> scaffoldMessengerKey =
    GlobalKey<ScaffoldMessengerState>();

void main() async {
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
      navigatorKey: navigatorKey,
      scaffoldMessengerKey: scaffoldMessengerKey,
      title: 'Sessio',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.pink),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.pink,
      ),
      themeMode: ThemeMode.system,
      home: const MyHomePage(title: 'Sessio'),
    );
  }
}
