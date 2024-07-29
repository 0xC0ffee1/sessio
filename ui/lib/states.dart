import 'package:equatable/equatable.dart';

class ConnectionState extends Equatable {
  final List<ConnectionTab> tabs;

  ConnectionState(this.tabs);

  @override
  List<Object?> get props => [tabs];
}

class ConnectionTab extends Equatable {
  final String title;
  final String clientId;
  final bool connected;

  ConnectionTab({
    required this.title,
    required this.clientId,
    this.connected = false,
  });

  @override
  List<Object?> get props => [title, clientId, connected];
}
