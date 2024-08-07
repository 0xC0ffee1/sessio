import 'dart:io';

import 'package:flutter/material.dart';

abstract class FileBrowser with ChangeNotifier {
  List<String> get currentPath;
  List<FileMeta> get currentFiles;
  Future<List<FileMeta>> refreshFileList();
  bool get isLoading;

  void setCurrentPath(List<String> path);
  Stream<TransferStatus> addFile(String filePath, String fileName);
  Stream<TransferStatus> copyFile(String filePath, String dest);
  Future<void> navigateToDirectory(String directory);
  Future<void> navigateUp();

  void setCurrentTransferData(TransferData data);
  TransferData? getCurrentTransfer();
  void setTransferCancelled();
}

enum TransferStatusType {
  progress,
  completed,
}

class TransferData {
  final int fileSize;
  final Stream<TransferStatus> transferStream;

  const TransferData({required this.fileSize, required this.transferStream});
}

class TransferStatus {
  final TransferStatusType type;
  final int bytesRead;

  const TransferStatus.progress({required this.bytesRead})
      : type = TransferStatusType.progress;
  const TransferStatus.completed()
      : bytesRead = 0,
        type = TransferStatusType.completed;

  int getBytesRead() => bytesRead;
}

//Java moment
class FileMeta {
  final String filename;
  final String path;
  final int byteSize;
  final bool isDir;

  FileMeta(
      {required this.filename,
      required this.path,
      required this.byteSize,
      required this.isDir});

  String getFilename() => filename;
  String getPath() => path;
  int getByteSize() => byteSize;

  // Override the == operator to compare objects by value
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FileMeta &&
          runtimeType == other.runtimeType &&
          filename == other.filename &&
          path == other.path &&
          byteSize == other.byteSize &&
          isDir == other.isDir;

  // Override the hashCode method
  @override
  int get hashCode =>
      filename.hashCode ^ path.hashCode ^ byteSize.hashCode ^ isDir.hashCode;

  // Optionally, you can override the toString method for better debug output
  @override
  String toString() {
    return 'FileMeta{filename: $filename, path: $path, byteSize: $byteSize, dir: $isDir}';
  }
}

abstract class LocalFile {
  FileMeta getMeta();
  Future<File> getFileHandle();
}
