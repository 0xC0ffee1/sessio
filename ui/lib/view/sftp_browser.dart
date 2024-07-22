import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sessio_ui/model/sftp/browser.dart';
import 'package:sessio_ui/model/sftp/sftp.dart';
import 'sftp_browser.dart'; // Import the file containing your SftpBrowser class

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
              await _browser.addFile(
                  res.files.single.path!, res.files.single.name);
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
                      browser.copyFile(file.path, outputFile);
                    }
                  },
                  icon: Icon(Icons.download),
                ),
        );
      },
    );
  }
}
