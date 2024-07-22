import 'package:flutter/material.dart';
import 'package:xterm/xterm.dart';

class SftpBrowserState with ChangeNotifier {
  List<String> _currentPath = [];
  List<String> _fileList = [];
  bool _isLoading = false;

  List<String> get currentPath => _currentPath;
  List<String> get fileList => _fileList;
  bool get isLoading => _isLoading;

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
