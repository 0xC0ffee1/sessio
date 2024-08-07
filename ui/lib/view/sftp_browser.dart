import 'dart:io';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:material_symbols_icons/symbols.dart';

class FileTransferOverlay extends StatefulWidget {
  final int fileSize;
  final Stream<TransferStatus> transferStream;
  final VoidCallback onCancel;

  const FileTransferOverlay({
    Key? key,
    required this.fileSize,
    required this.transferStream,
    required this.onCancel,
  }) : super(key: key);

  @override
  _FileTransferOverlayState createState() => _FileTransferOverlayState();
}

class _FileTransferOverlayState extends State<FileTransferOverlay> {
  int previousBytesRead = 0;
  DateTime? previousTimestamp;

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black54,
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Card(
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: StreamBuilder<TransferStatus>(
                stream: widget.transferStream,
                builder: (context, snapshot) {
                  if (snapshot.connectionState == ConnectionState.waiting) {
                    return Text('Initializing...');
                  } else if (snapshot.hasError) {
                    return Text('Error: ${snapshot.error}');
                  } else if (!snapshot.hasData) {
                    widget.onCancel();
                    return Text('Completed');
                  } else {
                    final status = snapshot.data!;
                    final progress = status.bytesRead;

                    double speed = 0;
                    final timestamp = DateTime.now();
                    if (previousTimestamp != null) {
                      final elapsedTime = timestamp
                          .difference(previousTimestamp!)
                          .inMilliseconds;
                      final bytesTransferred =
                          status.bytesRead - previousBytesRead;
                      speed = bytesTransferred /
                          elapsedTime *
                          1000 /
                          (1024 * 1024); // Convert to MB/s
                    }

                    previousBytesRead = status.bytesRead;
                    previousTimestamp = timestamp;

                    if (status.type == TransferStatusType.progress) {
                      return Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Text('Transferring...'),
                          SizedBox(height: 20),
                          LinearProgressIndicator(
                            value: progress / widget.fileSize,
                          ),
                          SizedBox(height: 20),
                          Text(
                              '${(progress / widget.fileSize * 100).toStringAsFixed(2)}% completed'),
                          SizedBox(height: 10),
                          Text('Speed: ${speed.toStringAsFixed(2)} MB/s'),
                          SizedBox(height: 20),
                          ElevatedButton(
                            onPressed: widget.onCancel,
                            child: Text('Cancel'),
                          ),
                        ],
                      );
                    } else if (status.type == TransferStatusType.completed) {
                      widget.onCancel();
                      return Text('Transfer completed');
                    } else {
                      return Text('Unknown state');
                    }
                  }
                },
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class FileBrowserView extends StatefulWidget {
  final FileBrowser _browser;

  const FileBrowserView({
    Key? key,
    required FileBrowser browser,
  })  : _browser = browser,
        super(key: key);

  @override
  _FileBrowserViewState createState() => _FileBrowserViewState();
}

class _FileBrowserViewState extends State<FileBrowserView>
    with AutomaticKeepAliveClientMixin {
  void _startFileTransfer(int fileSize, Stream<TransferStatus> transferStream) {
    setState(() {
      widget._browser.setCurrentTransferData(
          TransferData(fileSize: fileSize, transferStream: transferStream));
    });
  }

  void _cancelFileTransfer() {
    setState(() {
      widget._browser.setTransferCancelled();
    });
  }

  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context); // Ensure super.build is called
    return ChangeNotifierProvider<FileBrowser>.value(
      value: widget._browser,
      child: Scaffold(
        appBar: AppBar(
          title: Text('File Browser'),
          actions: [
            IconButton(
              icon: Icon(Icons.refresh),
              onPressed: () {
                widget._browser.refreshFileList();
              },
            ),
          ],
        ),
        body: Stack(
          children: [
            Column(
              children: [
                PathNavigator(),
                Expanded(
                  child: FileListView(
                    onFileTransferStart: _startFileTransfer,
                  ),
                ),
              ],
            ),
            if (widget._browser.getCurrentTransfer() != null)
              FileTransferOverlay(
                fileSize: widget._browser.getCurrentTransfer()!.fileSize,
                transferStream:
                    widget._browser.getCurrentTransfer()!.transferStream,
                onCancel: _cancelFileTransfer,
              ),
          ],
        ),
        floatingActionButton: FloatingActionButton(
          backgroundColor: Colors.pink,
          onPressed: () async {
            FilePickerResult? res = await FilePicker.platform.pickFiles(
              dialogTitle: 'Select file:',
            );
            if (res != null) {
              int fileSize = res.files.single.size;

              final transferStream = widget._browser
                  .addFile(res.files.single.path!, res.files.single.name);

              _startFileTransfer(fileSize, transferStream);
            }
          },
          child: Icon(Icons.add),
        ),
      ),
    );
  }
}

class PathNavigator extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final browser = context.watch<FileBrowser>();
    final path = browser.currentPath.join('/');

    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Row(
        children: [
          IconButton(
            icon: Icon(Icons.arrow_upward),
            onPressed: () {
              browser.navigateUp();
            },
          ),
          Expanded(
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Text(path.isEmpty ? '/' : path),
            ),
          ),
        ],
      ),
    );
  }
}

extension ByteSizeFormat on int {
  String formatBytes() {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    double size = this.toDouble();
    int unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return '${size.toStringAsFixed(1)} ${units[unitIndex]}';
  }
}

class FileListView extends StatelessWidget {
  final void Function(int fileSize, Stream<TransferStatus> transferStream)
      onFileTransferStart;

  const FileListView({
    Key? key,
    required this.onFileTransferStart,
  }) : super(key: key);

  Future<void> _handleFileDownload(
      FileMeta file, BuildContext context, FileBrowser browser) async {
    String? outputFile;
    if (Platform.isAndroid || Platform.isIOS) {
      outputFile = await FilePicker.platform.saveFile(
        dialogTitle: 'Please select an output file:',
        fileName: file.filename,
        bytes: Uint8List(0),
      );
    } else {
      outputFile = await FilePicker.platform.saveFile(
        dialogTitle: 'Please select an output file:',
        fileName: file.filename,
      );
    }
    if (outputFile != null) {
      int fileSize = file.byteSize;

      final transferStream = browser.copyFile(file.path, outputFile);

      onFileTransferStart(fileSize, transferStream);
    }
  }

  @override
  Widget build(BuildContext context) {
    final browser = context.watch<FileBrowser>();
    final files = browser.currentFiles;
    final isLoading = browser.isLoading;

    if (isLoading) {
      return Center(child: CircularProgressIndicator());
    }

    if (files.isEmpty) {
      return Center(child: Text('No files found.'));
    }

    return ListView.builder(
      itemCount: files.length,
      itemBuilder: (context, index) {
        final file = files[index];
        return ListTile(
          leading: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Checkbox(
                value: false,
                onChanged: (bool? value) {},
              ),
              SizedBox(width: 10),
              Icon(
                file.isDir ? Icons.folder : Icons.insert_drive_file_outlined,
              ),
            ],
          ),
          title: Text(file.filename),
          subtitle: Text(file.isDir ? " " : file.byteSize.formatBytes()),
          onTap: () {
            if (file.isDir) {
              browser.navigateToDirectory(file.filename);
            } else {
              // Maybe open built in editor
            }
          },
          trailing: file.isDir
              ? null
              : PopupMenuButton<String>(
                  onSelected: (String result) async {
                    switch (result) {
                      case "download":
                        _handleFileDownload(file, context, browser);
                        break;
                      case "delete":
                        // Handle delete
                        break;
                      case "rename":
                        // Handle rename
                        break;
                    }
                  },
                  itemBuilder: (BuildContext context) =>
                      <PopupMenuEntry<String>>[
                    const PopupMenuItem<String>(
                      value: 'download',
                      child: Row(
                        children: [
                          Icon(Symbols.download),
                          SizedBox(width: 10),
                          Text('Download')
                        ],
                      ),
                    ),
                    const PopupMenuItem<String>(
                      value: 'delete',
                      child: Row(
                        children: [
                          Icon(Symbols.delete),
                          SizedBox(width: 10),
                          Text('Delete')
                        ],
                      ),
                    ),
                    const PopupMenuItem<String>(
                      value: 'rename',
                      child: Row(
                        children: [
                          Icon(Symbols.edit),
                          SizedBox(width: 10),
                          Text('Rename')
                        ],
                      ),
                    ),
                  ],
                ),
        );
      },
    );
  }
}
