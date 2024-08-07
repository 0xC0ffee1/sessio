import 'package:flutter/material.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:xterm/xterm.dart';

class SftpBrowserState with ChangeNotifier {
  late Stream<TransferStatus> _currentTransferStream;

  List<String> _currentPath = [];
  List<String> _fileList = [];
  bool _isLoading = false;

  List<String> get currentPath => _currentPath;
  List<String> get fileList => _fileList;
  Stream<TransferStatus> get currentTransferStream => _currentTransferStream;
  bool get isLoading => _isLoading;

  void setCurrentTransferStream(Stream<TransferStatus> stream) {
    this._currentTransferStream = stream;
  }

  void setCurrentPath(List<String> path) {
    _currentPath = path;
    notifyListeners();
  }

  void addFile(String fileName) async {
    _fileList.add(fileName);

    _setLoading(false);
    notifyListeners();
  }

  void _setLoading(bool loading) {
    _isLoading = loading;
    notifyListeners();
  }

  void navigateToDirectory(String directory) {
    _currentPath.add(directory);
  }

  void navigateUp() {
    if (_currentPath.isNotEmpty) {
      _currentPath.removeLast();
    }
  }
}
