//
//  Generated code. Do not modify.
//  source: client_ipc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use sftpRequestDescriptor instead')
const SftpRequest$json = {
  '1': 'SftpRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `SftpRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sftpRequestDescriptor = $convert.base64Decode(
    'CgtTZnRwUmVxdWVzdBIdCgpzZXNzaW9uX2lkGAEgASgJUglzZXNzaW9uSWQ=');

@$core.Deprecated('Use sftpRequestResponseDescriptor instead')
const SftpRequestResponse$json = {
  '1': 'SftpRequestResponse',
  '2': [
    {'1': 'channel_id', '3': 1, '4': 1, '5': 9, '10': 'channelId'},
  ],
};

/// Descriptor for `SftpRequestResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sftpRequestResponseDescriptor = $convert.base64Decode(
    'ChNTZnRwUmVxdWVzdFJlc3BvbnNlEh0KCmNoYW5uZWxfaWQYASABKAlSCWNoYW5uZWxJZA==');

@$core.Deprecated('Use msgDescriptor instead')
const Msg$json = {
  '1': 'Msg',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 11, '6': '.clientipc.Msg.Data', '9': 0, '10': 'data'},
    {'1': 'pty_request', '3': 2, '4': 1, '5': 11, '6': '.clientipc.Msg.PtyRequest', '9': 0, '10': 'ptyRequest'},
    {'1': 'shell_request', '3': 3, '4': 1, '5': 11, '6': '.clientipc.Msg.ShellRequest', '9': 0, '10': 'shellRequest'},
    {'1': 'channel_init', '3': 4, '4': 1, '5': 11, '6': '.clientipc.Msg.ChannelInit', '9': 0, '10': 'channelInit'},
  ],
  '3': [Msg_Data$json, Msg_PtyRequest$json, Msg_ShellRequest$json, Msg_ChannelInit$json],
  '8': [
    {'1': 'type'},
  ],
};

@$core.Deprecated('Use msgDescriptor instead')
const Msg_Data$json = {
  '1': 'Data',
  '2': [
    {'1': 'payload', '3': 1, '4': 1, '5': 12, '10': 'payload'},
  ],
};

@$core.Deprecated('Use msgDescriptor instead')
const Msg_PtyRequest$json = {
  '1': 'PtyRequest',
  '2': [
    {'1': 'col_width', '3': 1, '4': 1, '5': 13, '10': 'colWidth'},
    {'1': 'row_height', '3': 2, '4': 1, '5': 13, '10': 'rowHeight'},
  ],
};

@$core.Deprecated('Use msgDescriptor instead')
const Msg_ShellRequest$json = {
  '1': 'ShellRequest',
};

@$core.Deprecated('Use msgDescriptor instead')
const Msg_ChannelInit$json = {
  '1': 'ChannelInit',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `Msg`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List msgDescriptor = $convert.base64Decode(
    'CgNNc2cSKQoEZGF0YRgBIAEoCzITLmNsaWVudGlwYy5Nc2cuRGF0YUgAUgRkYXRhEjwKC3B0eV'
    '9yZXF1ZXN0GAIgASgLMhkuY2xpZW50aXBjLk1zZy5QdHlSZXF1ZXN0SABSCnB0eVJlcXVlc3QS'
    'QgoNc2hlbGxfcmVxdWVzdBgDIAEoCzIbLmNsaWVudGlwYy5Nc2cuU2hlbGxSZXF1ZXN0SABSDH'
    'NoZWxsUmVxdWVzdBI/CgxjaGFubmVsX2luaXQYBCABKAsyGi5jbGllbnRpcGMuTXNnLkNoYW5u'
    'ZWxJbml0SABSC2NoYW5uZWxJbml0GiAKBERhdGESGAoHcGF5bG9hZBgBIAEoDFIHcGF5bG9hZB'
    'pICgpQdHlSZXF1ZXN0EhsKCWNvbF93aWR0aBgBIAEoDVIIY29sV2lkdGgSHQoKcm93X2hlaWdo'
    'dBgCIAEoDVIJcm93SGVpZ2h0Gg4KDFNoZWxsUmVxdWVzdBosCgtDaGFubmVsSW5pdBIdCgpzZX'
    'NzaW9uX2lkGAEgASgJUglzZXNzaW9uSWRCBgoEdHlwZQ==');

@$core.Deprecated('Use listDirDescriptor instead')
const ListDir$json = {
  '1': 'ListDir',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'session_id', '3': 2, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `ListDir`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listDirDescriptor = $convert.base64Decode(
    'CgdMaXN0RGlyEhIKBHBhdGgYASABKAlSBHBhdGgSHQoKc2Vzc2lvbl9pZBgCIAEoCVIJc2Vzc2'
    'lvbklk');

@$core.Deprecated('Use pathDescriptor instead')
const Path$json = {
  '1': 'Path',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'session_id', '3': 2, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `Path`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pathDescriptor = $convert.base64Decode(
    'CgRQYXRoEhIKBHBhdGgYASABKAlSBHBhdGgSHQoKc2Vzc2lvbl9pZBgCIAEoCVIJc2Vzc2lvbk'
    'lk');

@$core.Deprecated('Use fileTransferRequestDescriptor instead')
const FileTransferRequest$json = {
  '1': 'FileTransferRequest',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'remote_path', '3': 2, '4': 1, '5': 9, '10': 'remotePath'},
    {'1': 'local_path', '3': 3, '4': 1, '5': 9, '10': 'localPath'},
  ],
};

/// Descriptor for `FileTransferRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileTransferRequestDescriptor = $convert.base64Decode(
    'ChNGaWxlVHJhbnNmZXJSZXF1ZXN0Eh0KCnNlc3Npb25faWQYASABKAlSCXNlc3Npb25JZBIfCg'
    'tyZW1vdGVfcGF0aBgCIAEoCVIKcmVtb3RlUGF0aBIdCgpsb2NhbF9wYXRoGAMgASgJUglsb2Nh'
    'bFBhdGg=');

@$core.Deprecated('Use fileTransferResponseDescriptor instead')
const FileTransferResponse$json = {
  '1': 'FileTransferResponse',
  '2': [
    {'1': 'local_path', '3': 1, '4': 1, '5': 9, '10': 'localPath'},
  ],
};

/// Descriptor for `FileTransferResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileTransferResponseDescriptor = $convert.base64Decode(
    'ChRGaWxlVHJhbnNmZXJSZXNwb25zZRIdCgpsb2NhbF9wYXRoGAEgASgJUglsb2NhbFBhdGg=');

@$core.Deprecated('Use fileWriteRequestDescriptor instead')
const FileWriteRequest$json = {
  '1': 'FileWriteRequest',
  '2': [
    {'1': 'file_handle_id', '3': 1, '4': 1, '5': 9, '10': 'fileHandleId'},
    {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
    {'1': 'session_id', '3': 3, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `FileWriteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileWriteRequestDescriptor = $convert.base64Decode(
    'ChBGaWxlV3JpdGVSZXF1ZXN0EiQKDmZpbGVfaGFuZGxlX2lkGAEgASgJUgxmaWxlSGFuZGxlSW'
    'QSEgoEZGF0YRgCIAEoDFIEZGF0YRIdCgpzZXNzaW9uX2lkGAMgASgJUglzZXNzaW9uSWQ=');

@$core.Deprecated('Use fileWriteResponseDescriptor instead')
const FileWriteResponse$json = {
  '1': 'FileWriteResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `FileWriteResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileWriteResponseDescriptor = $convert.base64Decode(
    'ChFGaWxlV3JpdGVSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

@$core.Deprecated('Use fileReadRequestDescriptor instead')
const FileReadRequest$json = {
  '1': 'FileReadRequest',
  '2': [
    {'1': 'file_handle_id', '3': 1, '4': 1, '5': 9, '10': 'fileHandleId'},
    {'1': 'buf_size', '3': 2, '4': 1, '5': 5, '10': 'bufSize'},
    {'1': 'session_id', '3': 3, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `FileReadRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileReadRequestDescriptor = $convert.base64Decode(
    'Cg9GaWxlUmVhZFJlcXVlc3QSJAoOZmlsZV9oYW5kbGVfaWQYASABKAlSDGZpbGVIYW5kbGVJZB'
    'IZCghidWZfc2l6ZRgCIAEoBVIHYnVmU2l6ZRIdCgpzZXNzaW9uX2lkGAMgASgJUglzZXNzaW9u'
    'SWQ=');

@$core.Deprecated('Use fileReadResponseDescriptor instead')
const FileReadResponse$json = {
  '1': 'FileReadResponse',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `FileReadResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileReadResponseDescriptor = $convert.base64Decode(
    'ChBGaWxlUmVhZFJlc3BvbnNlEhIKBGRhdGEYASABKAxSBGRhdGE=');

@$core.Deprecated('Use fileCloseResponseDescriptor instead')
const FileCloseResponse$json = {
  '1': 'FileCloseResponse',
  '2': [
    {'1': 'success', '3': 1, '4': 1, '5': 8, '10': 'success'},
  ],
};

/// Descriptor for `FileCloseResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileCloseResponseDescriptor = $convert.base64Decode(
    'ChFGaWxlQ2xvc2VSZXNwb25zZRIYCgdzdWNjZXNzGAEgASgIUgdzdWNjZXNz');

@$core.Deprecated('Use fileMetadataRequestDescriptor instead')
const FileMetadataRequest$json = {
  '1': 'FileMetadataRequest',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'session_id', '3': 2, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `FileMetadataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileMetadataRequestDescriptor = $convert.base64Decode(
    'ChNGaWxlTWV0YWRhdGFSZXF1ZXN0EhIKBHBhdGgYASABKAlSBHBhdGgSHQoKc2Vzc2lvbl9pZB'
    'gCIAEoCVIJc2Vzc2lvbklk');

@$core.Deprecated('Use fileMetadataResponseDescriptor instead')
const FileMetadataResponse$json = {
  '1': 'FileMetadataResponse',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    {'1': 'size', '3': 2, '4': 1, '5': 4, '10': 'size'},
    {'1': 'last_modified', '3': 3, '4': 1, '5': 4, '10': 'lastModified'},
    {'1': 'is_directory', '3': 4, '4': 1, '5': 8, '10': 'isDirectory'},
  ],
};

/// Descriptor for `FileMetadataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileMetadataResponseDescriptor = $convert.base64Decode(
    'ChRGaWxlTWV0YWRhdGFSZXNwb25zZRISCgRwYXRoGAEgASgJUgRwYXRoEhIKBHNpemUYAiABKA'
    'RSBHNpemUSIwoNbGFzdF9tb2RpZmllZBgDIAEoBFIMbGFzdE1vZGlmaWVkEiEKDGlzX2RpcmVj'
    'dG9yeRgEIAEoCFILaXNEaXJlY3Rvcnk=');

@$core.Deprecated('Use dirMetadataDescriptor instead')
const DirMetadata$json = {
  '1': 'DirMetadata',
  '2': [
    {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
  ],
};

/// Descriptor for `DirMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List dirMetadataDescriptor = $convert.base64Decode(
    'CgtEaXJNZXRhZGF0YRISCgRwYXRoGAEgASgJUgRwYXRo');

@$core.Deprecated('Use fileListDescriptor instead')
const FileList$json = {
  '1': 'FileList',
  '2': [
    {'1': 'files', '3': 1, '4': 3, '5': 11, '6': '.clientipc.FileData', '10': 'files'},
  ],
};

/// Descriptor for `FileList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileListDescriptor = $convert.base64Decode(
    'CghGaWxlTGlzdBIpCgVmaWxlcxgBIAMoCzITLmNsaWVudGlwYy5GaWxlRGF0YVIFZmlsZXM=');

@$core.Deprecated('Use fileDataDescriptor instead')
const FileData$json = {
  '1': 'FileData',
  '2': [
    {'1': 'file_name', '3': 1, '4': 1, '5': 9, '10': 'fileName'},
    {'1': 'file_size', '3': 2, '4': 1, '5': 4, '10': 'fileSize'},
    {'1': 'file_path', '3': 3, '4': 1, '5': 9, '10': 'filePath'},
    {'1': 'is_dir', '3': 4, '4': 1, '5': 8, '10': 'isDir'},
  ],
};

/// Descriptor for `FileData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDataDescriptor = $convert.base64Decode(
    'CghGaWxlRGF0YRIbCglmaWxlX25hbWUYASABKAlSCGZpbGVOYW1lEhsKCWZpbGVfc2l6ZRgCIA'
    'EoBFIIZmlsZVNpemUSGwoJZmlsZV9wYXRoGAMgASgJUghmaWxlUGF0aBIVCgZpc19kaXIYBCAB'
    'KAhSBWlzRGly');

@$core.Deprecated('Use ptyRequestResponseDescriptor instead')
const PtyRequestResponse$json = {
  '1': 'PtyRequestResponse',
  '2': [
    {'1': 'channel_id', '3': 1, '4': 1, '5': 9, '10': 'channelId'},
  ],
};

/// Descriptor for `PtyRequestResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List ptyRequestResponseDescriptor = $convert.base64Decode(
    'ChJQdHlSZXF1ZXN0UmVzcG9uc2USHQoKY2hhbm5lbF9pZBgBIAEoCVIJY2hhbm5lbElk');

@$core.Deprecated('Use genKeysRequestDescriptor instead')
const GenKeysRequest$json = {
  '1': 'GenKeysRequest',
  '2': [
    {'1': 'key_path', '3': 1, '4': 1, '5': 9, '10': 'keyPath'},
  ],
};

/// Descriptor for `GenKeysRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List genKeysRequestDescriptor = $convert.base64Decode(
    'Cg5HZW5LZXlzUmVxdWVzdBIZCghrZXlfcGF0aBgBIAEoCVIHa2V5UGF0aA==');

@$core.Deprecated('Use genKeysResponseDescriptor instead')
const GenKeysResponse$json = {
  '1': 'GenKeysResponse',
  '2': [
    {'1': 'key_path', '3': 1, '4': 1, '5': 9, '10': 'keyPath'},
  ],
};

/// Descriptor for `GenKeysResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List genKeysResponseDescriptor = $convert.base64Decode(
    'Cg9HZW5LZXlzUmVzcG9uc2USGQoIa2V5X3BhdGgYASABKAlSB2tleVBhdGg=');

@$core.Deprecated('Use streamRequestDescriptor instead')
const StreamRequest$json = {
  '1': 'StreamRequest',
  '2': [
    {'1': 'session_id', '3': 2, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `StreamRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List streamRequestDescriptor = $convert.base64Decode(
    'Cg1TdHJlYW1SZXF1ZXN0Eh0KCnNlc3Npb25faWQYAiABKAlSCXNlc3Npb25JZBISCgRkYXRhGA'
    'EgASgMUgRkYXRh');

@$core.Deprecated('Use streamResponseDescriptor instead')
const StreamResponse$json = {
  '1': 'StreamResponse',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `StreamResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List streamResponseDescriptor = $convert.base64Decode(
    'Cg5TdHJlYW1SZXNwb25zZRISCgRkYXRhGAEgASgMUgRkYXRh');

@$core.Deprecated('Use newSessionRequestDescriptor instead')
const NewSessionRequest$json = {
  '1': 'NewSessionRequest',
  '2': [
    {'1': 'connection_id', '3': 1, '4': 1, '5': 9, '10': 'connectionId'},
    {'1': 'username', '3': 2, '4': 1, '5': 9, '10': 'username'},
    {'1': 'private_key', '3': 3, '4': 1, '5': 9, '10': 'privateKey'},
    {'1': 'known_hosts_path', '3': 4, '4': 1, '5': 9, '10': 'knownHostsPath'},
  ],
};

/// Descriptor for `NewSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSessionRequestDescriptor = $convert.base64Decode(
    'ChFOZXdTZXNzaW9uUmVxdWVzdBIjCg1jb25uZWN0aW9uX2lkGAEgASgJUgxjb25uZWN0aW9uSW'
    'QSGgoIdXNlcm5hbWUYAiABKAlSCHVzZXJuYW1lEh8KC3ByaXZhdGVfa2V5GAMgASgJUgpwcml2'
    'YXRlS2V5EigKEGtub3duX2hvc3RzX3BhdGgYBCABKAlSDmtub3duSG9zdHNQYXRo');

@$core.Deprecated('Use newSessionResponseDescriptor instead')
const NewSessionResponse$json = {
  '1': 'NewSessionResponse',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `NewSessionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSessionResponseDescriptor = $convert.base64Decode(
    'ChJOZXdTZXNzaW9uUmVzcG9uc2USHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklk');

@$core.Deprecated('Use newConnectionRequestDescriptor instead')
const NewConnectionRequest$json = {
  '1': 'NewConnectionRequest',
  '2': [
    {'1': 'coordinator_url', '3': 1, '4': 1, '5': 9, '10': 'coordinatorUrl'},
    {'1': 'target_id', '3': 2, '4': 1, '5': 9, '10': 'targetId'},
  ],
};

/// Descriptor for `NewConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newConnectionRequestDescriptor = $convert.base64Decode(
    'ChROZXdDb25uZWN0aW9uUmVxdWVzdBInCg9jb29yZGluYXRvcl91cmwYASABKAlSDmNvb3JkaW'
    '5hdG9yVXJsEhsKCXRhcmdldF9pZBgCIAEoCVIIdGFyZ2V0SWQ=');

@$core.Deprecated('Use newConnectionResponseDescriptor instead')
const NewConnectionResponse$json = {
  '1': 'NewConnectionResponse',
  '2': [
    {'1': 'connection_id', '3': 1, '4': 1, '5': 9, '10': 'connectionId'},
  ],
};

/// Descriptor for `NewConnectionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newConnectionResponseDescriptor = $convert.base64Decode(
    'ChVOZXdDb25uZWN0aW9uUmVzcG9uc2USIwoNY29ubmVjdGlvbl9pZBgBIAEoCVIMY29ubmVjdG'
    'lvbklk');

