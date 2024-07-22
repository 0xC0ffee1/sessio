import 'package:flutter/material.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';

class SftpBrowser with ChangeNotifier implements FileBrowser {
  //fix path always starting with . and other stuff with it
  List<String> _currentPath = ["."];
  List<FileMeta> _currentFiles = [];
  bool _isLoadingFiles = false;

  final ClientIPCClient _client;
  final String _sessionId;

  SftpBrowser(this._client, this._sessionId);

  //Uploads from local machine to remote
  @override
  Future<void> addFile(String localPath, String fileName) async {
    await _client.fileUpload(FileTransferRequest(
        sessionId: _sessionId,
        localPath: localPath,
        remotePath: "${_currentPath.join('/')}$fileName"));

    await refreshFileList();
  }

  //Downloads from remote machine to local
  @override
  Future<void> copyFile(String filePath, String dest) async {
    await _client.fileDownload(FileTransferRequest(
        sessionId: _sessionId, localPath: dest, remotePath: filePath));
    await refreshFileList();
  }

  @override
  List<String> get currentPath => _currentPath;

  @override
  Future<List<FileMeta>> refreshFileList() async {
    _isLoadingFiles = true;
    FileList remoteList = await _client.listDirectory(
        Path(sessionId: _sessionId, path: _currentPath.join("/")));

    var res = remoteList.files.map((file) {
      return FileMeta(
          filename: file.fileName,
          path: file.filePath,
          byteSize: file.fileSize.toInt(),
          isDir: file.isDir);
    }).toList();
    print("FETCHED ${res.length}");
    _currentFiles = res;
    _isLoadingFiles = false;
    notifyListeners();
    return res;
  }

  @override
  bool get isLoading => _isLoadingFiles;

  @override
  Future<void> navigateToDirectory(String directory) async {
    _currentPath.add(directory);
    refreshFileList();
  }

  @override
  Future<void> navigateUp() async {
    _currentPath.removeLast();
    refreshFileList();
  }

  @override
  Future<void> setCurrentPath(List<String> path) async {
    _currentPath = path;
    refreshFileList();
  }

  @override
  List<FileMeta> get currentFiles => _currentFiles;
}
