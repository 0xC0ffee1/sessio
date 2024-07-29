import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'sftp_browser.dart'; // Import the file containing your SftpBrowser class

void showProgressDialog(
    BuildContext context, int fileSize, Stream<TransferStatus> transferStream) {
  int previousBytesRead = 0;
  DateTime? previousTimestamp;

  showDialog(
    context: context,
    barrierDismissible: false,
    builder: (context) {
      return AlertDialog(
        title: Text('File Transfer'),
        content: StreamBuilder<TransferStatus>(
          stream: transferStream,
          builder: (context, snapshot) {
            if (snapshot.connectionState == ConnectionState.waiting) {
              return Text('Initializing...');
            } else if (snapshot.hasError) {
              return Text('Error: ${snapshot.error}');
            } else if (!snapshot.hasData) {
              return Text('Unknown state');
            } else {
              final status = snapshot.data!;
              final progress = status.bytesRead;

              double speed = 0;
              final timestamp = DateTime.now();
              if (previousTimestamp != null) {
                final elapsedTime = timestamp.difference(previousTimestamp!).inMilliseconds;
                final bytesTransferred = status.bytesRead - previousBytesRead;
                speed = bytesTransferred / elapsedTime * 1000 / (1024 * 1024); // Convert to MB/s
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
                      value: progress / fileSize,
                    ),
                    SizedBox(height: 20),
                    Text('${(progress / fileSize * 100).toStringAsFixed(2)}% completed'),
                    SizedBox(height: 10),
                    Text('Speed: ${speed.toStringAsFixed(2)} MB/s'),
                  ],
                );
              } else if (status.type == TransferStatusType.completed) {
                Navigator.of(context).pop(); // Close the dialog on completion
                return Text('Transfer completed');
              } else {
                return Text('Unknown state');
              }
            }
          },
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
            },
            child: Text('Cancel'),
          ),
        ],
      );
    },
  );
}

class FileBrowserView extends StatelessWidget {
  final FileBrowser _browser;

  const FileBrowserView({
    Key? key,
    required FileBrowser browser,
  })  : _browser = browser,
        super(key: key);

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider<FileBrowser>.value(
      value: _browser,
      child: Scaffold(
        appBar: AppBar(
          title: Text('File Browser'),
          actions: [
            IconButton(
              icon: Icon(Icons.refresh),
              onPressed: () {
                _browser.refreshFileList();
              },
            ),
          ],
        ),
        body: Column(
          children: [
            PathNavigator(),
            Expanded(
              child: FileListView(),
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

              final transferStream = _browser.addFile(
                  res.files.single.path!, res.files.single.name);

              showProgressDialog(context, fileSize, transferStream);
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

class FileListView extends StatelessWidget {
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
          leading: Icon(file.isDir
              ? Icons.folder_outlined
              : Icons.insert_drive_file_outlined),
          title: Text(file.filename),
          subtitle: Text(file.path),
          onTap: () {
            if (file.isDir) {
              browser.navigateToDirectory(file.path);
            } else {
              // Handle file selection
            }
          },
          trailing: file.isDir
              ? null
              : IconButton(
                  onPressed: () async {
                    String? outputFile = await FilePicker.platform.saveFile(
                      dialogTitle: 'Please select an output file:',
                      fileName: file.filename,
                    );

                    if (outputFile != null) {
                      int fileSize = file.byteSize;

                      final transferStream =
                          browser.copyFile(file.path, outputFile);

                      showProgressDialog(context, fileSize, transferStream);
                    }
                  },
                  icon: Icon(Icons.download),
                ),
        );
      },
    );
  }
}
