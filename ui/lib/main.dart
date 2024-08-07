import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_background_service/flutter_background_service.dart';
import 'package:path_provider/path_provider.dart';
import 'package:sessio_ui/grpc_service.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'package:sessio_ui/model/terminal_state.dart';
import 'package:sessio_ui/view/homepage.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/view/sftp_browser.dart';
import 'package:workmanager/workmanager.dart';

final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
final GlobalKey<ScaffoldMessengerState> scaffoldMessengerKey =
    GlobalKey<ScaffoldMessengerState>();

typedef StartGrpcServerNative = Void Function(Pointer<Utf8> path);
typedef StartGrpcServer = void Function(Pointer<Utf8> path);

class NativeGrpcServer {
  late final DynamicLibrary _lib;

  NativeGrpcServer() {
    if (Platform.isAndroid) {
      _lib = DynamicLibrary.open('libgrpc_server.so');
    } else if (Platform.isIOS) {
      _lib = DynamicLibrary.process();
    } else {
      throw UnsupportedError('This platform is not supported.');
    }
  }

  // Bind the native function
  late final StartGrpcServer startGrpcServer = _lib
      .lookup<NativeFunction<StartGrpcServerNative>>('start_grpc_server')
      .asFunction();
}

@pragma('vm:entry-point')
Future<void> onStart(ServiceInstance service) async {
  Directory appDir = await getApplicationSupportDirectory();
  startGrpcServer(appDir.path + "/sessio.socket");
}

void startGrpcServer(String path) {
  final nativeAdd = NativeGrpcServer();
  final pathPointer = path.toNativeUtf8();
  nativeAdd.startGrpcServer(pathPointer);
  malloc.free(pathPointer);
}

@pragma(
    'vm:entry-point') // Mandatory if the App is obfuscated or using Flutter 3.1+
void callbackDispatcher() {
  Workmanager().executeTask((task, inputData) async {
    print(
        "Native called background task: $task"); //simpleTask will be emitted here.
    Directory appDir = await getApplicationSupportDirectory();
    startGrpcServer(appDir.path + "/sessio.socket");

    return Future.value(true);
  });
}

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
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
        primarySwatch: Colors.blue,
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
            seedColor: Colors.pink, brightness: Brightness.dark),
        primarySwatch: Colors.pink,
        brightness: Brightness.dark,
      ),
      themeMode: ThemeMode.system,
      home: const MyHomePage(title: 'Sessio'),
    );
  }
}
