import 'package:flutter/material.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:sessio_ui/src/generated/client_ipc.pbgrpc.dart';

class SftpBrowser with ChangeNotifier implements FileBrowser {
  //fix path always starting with . and other stuff with it
  List<String> _currentPath = [];
  List<FileMeta> _currentFiles = [];
  bool _isLoadingFiles = false;

  final ClientIPCClient _client;
  final String _sessionId;

  SftpBrowser(this._client, this._sessionId);

  //Uploads from local machine to remote
  @override
  Stream<TransferStatus> addFile(String localPath, String fileName) async* {
    final remotePath = _currentPath.isEmpty ? fileName : "${_currentPath.join('/')}/$fileName";
    final responseStream = _client.fileUpload(FileTransferRequest(
        sessionId: _sessionId,
        localPath: localPath,
        remotePath: remotePath));

    await for (var response in responseStream) {
      // Map the response to FileTransferStatus DTO
      switch (response.whichTyp()) {
        case FileTransferStatus_Typ.progress:
          yield TransferStatus.progress(bytesRead: response.progress.bytesRead);
          break;
        case FileTransferStatus_Typ.completed:
          yield TransferStatus.completed();
          break;
        case FileTransferStatus_Typ.notSet:
          break;
      }
    }
  }

  @override
  Stream<TransferStatus> copyFile(String filePath, String dest) async* {
    final responseStream = _client.fileDownload(FileTransferRequest(
      sessionId: _sessionId,
      localPath: dest,
      remotePath: filePath,
    ));

    await for (var response in responseStream) {
      // Map the response to TransferStatus
      switch (response.whichTyp()) {
        case FileTransferStatus_Typ.progress:
          yield TransferStatus.progress(bytesRead: response.progress.bytesRead);
          break;
        case FileTransferStatus_Typ.completed:
          yield TransferStatus.completed();
          break;
        case FileTransferStatus_Typ.notSet:
          break;
      }
    }
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
