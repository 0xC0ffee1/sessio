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

@$core.Deprecated('Use sessionMapDescriptor instead')
const SessionMap$json = {
  '1': 'SessionMap',
  '2': [
    {
      '1': 'map',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.clientipc.SessionMap.MapEntry',
      '10': 'map'
    },
    {
      '1': 'parents',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.clientipc.SessionMap.ParentsEntry',
      '10': 'parents'
    },
  ],
  '3': [SessionMap_MapEntry$json, SessionMap_ParentsEntry$json],
};

@$core.Deprecated('Use sessionMapDescriptor instead')
const SessionMap_MapEntry$json = {
  '1': 'MapEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {
      '1': 'value',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData',
      '10': 'value'
    },
  ],
  '7': {'7': true},
};

@$core.Deprecated('Use sessionMapDescriptor instead')
const SessionMap_ParentsEntry$json = {
  '1': 'ParentsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {
      '1': 'value',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.DeviceStatus',
      '10': 'value'
    },
  ],
  '7': {'7': true},
};

/// Descriptor for `SessionMap`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionMapDescriptor = $convert.base64Decode(
    'CgpTZXNzaW9uTWFwEjAKA21hcBgBIAMoCzIeLmNsaWVudGlwYy5TZXNzaW9uTWFwLk1hcEVudH'
    'J5UgNtYXASPAoHcGFyZW50cxgCIAMoCzIiLmNsaWVudGlwYy5TZXNzaW9uTWFwLlBhcmVudHNF'
    'bnRyeVIHcGFyZW50cxpOCghNYXBFbnRyeRIQCgNrZXkYASABKAlSA2tleRIsCgV2YWx1ZRgCIA'
    'EoCzIWLmNsaWVudGlwYy5TZXNzaW9uRGF0YVIFdmFsdWU6AjgBGlMKDFBhcmVudHNFbnRyeRIQ'
    'CgNrZXkYASABKAlSA2tleRItCgV2YWx1ZRgCIAEoCzIXLmNsaWVudGlwYy5EZXZpY2VTdGF0dX'
    'NSBXZhbHVlOgI4AQ==');

@$core.Deprecated('Use deviceStatusDescriptor instead')
const DeviceStatus$json = {
  '1': 'DeviceStatus',
  '2': [
    {'1': 'connected', '3': 1, '4': 1, '5': 8, '10': 'connected'},
  ],
};

/// Descriptor for `DeviceStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deviceStatusDescriptor = $convert.base64Decode(
    'CgxEZXZpY2VTdGF0dXMSHAoJY29ubmVjdGVkGAEgASgIUgljb25uZWN0ZWQ=');

@$core.Deprecated('Use sessionRequestDescriptor instead')
const SessionRequest$json = {
  '1': 'SessionRequest',
  '2': [
    {'1': 'parent', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'parent', '17': true},
  ],
  '8': [
    {'1': '_parent'},
  ],
};

/// Descriptor for `SessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionRequestDescriptor = $convert.base64Decode(
    'Cg5TZXNzaW9uUmVxdWVzdBIbCgZwYXJlbnQYASABKAlIAFIGcGFyZW50iAEBQgkKB19wYXJlbn'
    'Q=');

@$core.Deprecated('Use sessionDataDescriptor instead')
const SessionData$json = {
  '1': 'SessionData',
  '2': [
    {
      '1': 'pty',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData.PTYSession',
      '9': 0,
      '10': 'pty'
    },
    {
      '1': 'sftp',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData.SFTPSession',
      '9': 0,
      '10': 'sftp'
    },
    {
      '1': 'lpf',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData.LPFSession',
      '9': 0,
      '10': 'lpf'
    },
    {'1': 'session_id', '3': 4, '4': 1, '5': 9, '10': 'sessionId'},
    {'1': 'username', '3': 5, '4': 1, '5': 9, '10': 'username'},
    {'1': 'device_id', '3': 6, '4': 1, '5': 9, '10': 'deviceId'},
  ],
  '3': [
    SessionData_PTYSession$json,
    SessionData_SFTPSession$json,
    SessionData_LPFSession$json
  ],
  '8': [
    {'1': 'kind'},
  ],
};

@$core.Deprecated('Use sessionDataDescriptor instead')
const SessionData_PTYSession$json = {
  '1': 'PTYSession',
};

@$core.Deprecated('Use sessionDataDescriptor instead')
const SessionData_SFTPSession$json = {
  '1': 'SFTPSession',
};

@$core.Deprecated('Use sessionDataDescriptor instead')
const SessionData_LPFSession$json = {
  '1': 'LPFSession',
  '2': [
    {'1': 'local_host', '3': 1, '4': 1, '5': 9, '10': 'localHost'},
    {'1': 'local_port', '3': 2, '4': 1, '5': 13, '10': 'localPort'},
    {'1': 'remote_host', '3': 3, '4': 1, '5': 9, '10': 'remoteHost'},
    {'1': 'remote_port', '3': 4, '4': 1, '5': 13, '10': 'remotePort'},
  ],
};

/// Descriptor for `SessionData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sessionDataDescriptor = $convert.base64Decode(
    'CgtTZXNzaW9uRGF0YRI1CgNwdHkYASABKAsyIS5jbGllbnRpcGMuU2Vzc2lvbkRhdGEuUFRZU2'
    'Vzc2lvbkgAUgNwdHkSOAoEc2Z0cBgCIAEoCzIiLmNsaWVudGlwYy5TZXNzaW9uRGF0YS5TRlRQ'
    'U2Vzc2lvbkgAUgRzZnRwEjUKA2xwZhgDIAEoCzIhLmNsaWVudGlwYy5TZXNzaW9uRGF0YS5MUE'
    'ZTZXNzaW9uSABSA2xwZhIdCgpzZXNzaW9uX2lkGAQgASgJUglzZXNzaW9uSWQSGgoIdXNlcm5h'
    'bWUYBSABKAlSCHVzZXJuYW1lEhsKCWRldmljZV9pZBgGIAEoCVIIZGV2aWNlSWQaDAoKUFRZU2'
    'Vzc2lvbhoNCgtTRlRQU2Vzc2lvbhqMAQoKTFBGU2Vzc2lvbhIdCgpsb2NhbF9ob3N0GAEgASgJ'
    'Uglsb2NhbEhvc3QSHQoKbG9jYWxfcG9ydBgCIAEoDVIJbG9jYWxQb3J0Eh8KC3JlbW90ZV9ob3'
    'N0GAMgASgJUgpyZW1vdGVIb3N0Eh8KC3JlbW90ZV9wb3J0GAQgASgNUgpyZW1vdGVQb3J0QgYK'
    'BGtpbmQ=');

@$core.Deprecated('Use localPortForwardResponseDescriptor instead')
const LocalPortForwardResponse$json = {
  '1': 'LocalPortForwardResponse',
};

/// Descriptor for `LocalPortForwardResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List localPortForwardResponseDescriptor =
    $convert.base64Decode('ChhMb2NhbFBvcnRGb3J3YXJkUmVzcG9uc2U=');

@$core.Deprecated('Use localPortForwardRequestDescriptor instead')
const LocalPortForwardRequest$json = {
  '1': 'LocalPortForwardRequest',
  '2': [
    {'1': 'local_host', '3': 1, '4': 1, '5': 9, '10': 'localHost'},
    {'1': 'local_port', '3': 2, '4': 1, '5': 13, '10': 'localPort'},
    {'1': 'remote_host', '3': 3, '4': 1, '5': 9, '10': 'remoteHost'},
    {'1': 'remote_port', '3': 4, '4': 1, '5': 13, '10': 'remotePort'},
    {'1': 'session_id', '3': 5, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `LocalPortForwardRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List localPortForwardRequestDescriptor = $convert.base64Decode(
    'ChdMb2NhbFBvcnRGb3J3YXJkUmVxdWVzdBIdCgpsb2NhbF9ob3N0GAEgASgJUglsb2NhbEhvc3'
    'QSHQoKbG9jYWxfcG9ydBgCIAEoDVIJbG9jYWxQb3J0Eh8KC3JlbW90ZV9ob3N0GAMgASgJUgpy'
    'ZW1vdGVIb3N0Eh8KC3JlbW90ZV9wb3J0GAQgASgNUgpyZW1vdGVQb3J0Eh0KCnNlc3Npb25faW'
    'QYBSABKAlSCXNlc3Npb25JZA==');

@$core.Deprecated('Use fileDeleteRequestDescriptor instead')
const FileDeleteRequest$json = {
  '1': 'FileDeleteRequest',
  '2': [
    {'1': 'files', '3': 1, '4': 3, '5': 9, '10': 'files'},
  ],
};

/// Descriptor for `FileDeleteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileDeleteRequestDescriptor = $convert
    .base64Decode('ChFGaWxlRGVsZXRlUmVxdWVzdBIUCgVmaWxlcxgBIAMoCVIFZmlsZXM=');

@$core.Deprecated('Use getKeyRequestDescriptor instead')
const GetKeyRequest$json = {
  '1': 'GetKeyRequest',
};

/// Descriptor for `GetKeyRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getKeyRequestDescriptor =
    $convert.base64Decode('Cg1HZXRLZXlSZXF1ZXN0');

@$core.Deprecated('Use publicKeyDescriptor instead')
const PublicKey$json = {
  '1': 'PublicKey',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
  ],
};

/// Descriptor for `PublicKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List publicKeyDescriptor =
    $convert.base64Decode('CglQdWJsaWNLZXkSEAoDa2V5GAEgASgJUgNrZXk=');

@$core.Deprecated('Use initDataDescriptor instead')
const InitData$json = {
  '1': 'InitData',
  '2': [
    {'1': 'data_folder_path', '3': 1, '4': 1, '5': 9, '10': 'dataFolderPath'},
  ],
};

/// Descriptor for `InitData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List initDataDescriptor = $convert.base64Decode(
    'CghJbml0RGF0YRIoChBkYXRhX2ZvbGRlcl9wYXRoGAEgASgJUg5kYXRhRm9sZGVyUGF0aA==');

@$core.Deprecated('Use initResponseDescriptor instead')
const InitResponse$json = {
  '1': 'InitResponse',
};

/// Descriptor for `InitResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List initResponseDescriptor =
    $convert.base64Decode('CgxJbml0UmVzcG9uc2U=');

@$core.Deprecated('Use getSaveDataRequestDescriptor instead')
const GetSaveDataRequest$json = {
  '1': 'GetSaveDataRequest',
};

/// Descriptor for `GetSaveDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getSaveDataRequestDescriptor =
    $convert.base64Decode('ChJHZXRTYXZlRGF0YVJlcXVlc3Q=');

@$core.Deprecated('Use settingsRequestDescriptor instead')
const SettingsRequest$json = {
  '1': 'SettingsRequest',
};

/// Descriptor for `SettingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsRequestDescriptor =
    $convert.base64Decode('Cg9TZXR0aW5nc1JlcXVlc3Q=');

@$core.Deprecated('Use settingsDescriptor instead')
const Settings$json = {
  '1': 'Settings',
  '2': [
    {'1': 'coordinator_url', '3': 1, '4': 1, '5': 9, '10': 'coordinatorUrl'},
    {'1': 'device_id', '3': 2, '4': 1, '5': 9, '10': 'deviceId'},
  ],
};

/// Descriptor for `Settings`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List settingsDescriptor = $convert.base64Decode(
    'CghTZXR0aW5ncxInCg9jb29yZGluYXRvcl91cmwYASABKAlSDmNvb3JkaW5hdG9yVXJsEhsKCW'
    'RldmljZV9pZBgCIAEoCVIIZGV2aWNlSWQ=');

@$core.Deprecated('Use userDataDescriptor instead')
const UserData$json = {
  '1': 'UserData',
  '2': [
    {'1': 'used_device_ids', '3': 1, '4': 3, '5': 9, '10': 'usedDeviceIds'},
    {
      '1': 'saved_sessions',
      '3': 2,
      '4': 3,
      '5': 11,
      '6': '.clientipc.UserData.SavedSessionsEntry',
      '10': 'savedSessions'
    },
  ],
  '3': [UserData_SavedSessionsEntry$json],
};

@$core.Deprecated('Use userDataDescriptor instead')
const UserData_SavedSessionsEntry$json = {
  '1': 'SavedSessionsEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {
      '1': 'value',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData',
      '10': 'value'
    },
  ],
  '7': {'7': true},
};

/// Descriptor for `UserData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userDataDescriptor = $convert.base64Decode(
    'CghVc2VyRGF0YRImCg91c2VkX2RldmljZV9pZHMYASADKAlSDXVzZWREZXZpY2VJZHMSTQoOc2'
    'F2ZWRfc2Vzc2lvbnMYAiADKAsyJi5jbGllbnRpcGMuVXNlckRhdGEuU2F2ZWRTZXNzaW9uc0Vu'
    'dHJ5Ug1zYXZlZFNlc3Npb25zGlgKElNhdmVkU2Vzc2lvbnNFbnRyeRIQCgNrZXkYASABKAlSA2'
    'tleRIsCgV2YWx1ZRgCIAEoCzIWLmNsaWVudGlwYy5TZXNzaW9uRGF0YVIFdmFsdWU6AjgB');

@$core.Deprecated('Use emptyValueDescriptor instead')
const EmptyValue$json = {
  '1': 'EmptyValue',
};

/// Descriptor for `EmptyValue`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emptyValueDescriptor =
    $convert.base64Decode('CgpFbXB0eVZhbHVl');

@$core.Deprecated('Use valueDescriptor instead')
const Value$json = {
  '1': 'Value',
  '2': [
    {'1': 'string_value', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'stringValue'},
    {'1': 'int_value', '3': 2, '4': 1, '5': 5, '9': 0, '10': 'intValue'},
    {
      '1': 'json_value',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.clientipc.MapValue',
      '9': 0,
      '10': 'jsonValue'
    },
    {
      '1': 'list_value',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.clientipc.ListValue',
      '9': 0,
      '10': 'listValue'
    },
    {
      '1': 'empty',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.clientipc.EmptyValue',
      '9': 0,
      '10': 'empty'
    },
  ],
  '8': [
    {'1': 'kind'},
  ],
};

/// Descriptor for `Value`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List valueDescriptor = $convert.base64Decode(
    'CgVWYWx1ZRIjCgxzdHJpbmdfdmFsdWUYASABKAlIAFILc3RyaW5nVmFsdWUSHQoJaW50X3ZhbH'
    'VlGAIgASgFSABSCGludFZhbHVlEjQKCmpzb25fdmFsdWUYAyABKAsyEy5jbGllbnRpcGMuTWFw'
    'VmFsdWVIAFIJanNvblZhbHVlEjUKCmxpc3RfdmFsdWUYBCABKAsyFC5jbGllbnRpcGMuTGlzdF'
    'ZhbHVlSABSCWxpc3RWYWx1ZRItCgVlbXB0eRgFIAEoCzIVLmNsaWVudGlwYy5FbXB0eVZhbHVl'
    'SABSBWVtcHR5QgYKBGtpbmQ=');

@$core.Deprecated('Use mapValueDescriptor instead')
const MapValue$json = {
  '1': 'MapValue',
  '2': [
    {
      '1': 'map',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.clientipc.MapValue.MapEntry',
      '10': 'map'
    },
  ],
  '3': [MapValue_MapEntry$json],
};

@$core.Deprecated('Use mapValueDescriptor instead')
const MapValue_MapEntry$json = {
  '1': 'MapEntry',
  '2': [
    {'1': 'key', '3': 1, '4': 1, '5': 9, '10': 'key'},
    {
      '1': 'value',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Value',
      '10': 'value'
    },
  ],
  '7': {'7': true},
};

/// Descriptor for `MapValue`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mapValueDescriptor = $convert.base64Decode(
    'CghNYXBWYWx1ZRIuCgNtYXAYASADKAsyHC5jbGllbnRpcGMuTWFwVmFsdWUuTWFwRW50cnlSA2'
    '1hcBpICghNYXBFbnRyeRIQCgNrZXkYASABKAlSA2tleRImCgV2YWx1ZRgCIAEoCzIQLmNsaWVu'
    'dGlwYy5WYWx1ZVIFdmFsdWU6AjgB');

@$core.Deprecated('Use listValueDescriptor instead')
const ListValue$json = {
  '1': 'ListValue',
  '2': [
    {
      '1': 'list',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.clientipc.Value',
      '10': 'list'
    },
  ],
};

/// Descriptor for `ListValue`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listValueDescriptor = $convert.base64Decode(
    'CglMaXN0VmFsdWUSJAoEbGlzdBgBIAMoCzIQLmNsaWVudGlwYy5WYWx1ZVIEbGlzdA==');

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
    {
      '1': 'data',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Msg.Data',
      '9': 0,
      '10': 'data'
    },
    {
      '1': 'pty_request',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Msg.PtyRequest',
      '9': 0,
      '10': 'ptyRequest'
    },
    {
      '1': 'shell_request',
      '3': 3,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Msg.ShellRequest',
      '9': 0,
      '10': 'shellRequest'
    },
    {
      '1': 'channel_init',
      '3': 4,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Msg.ChannelInit',
      '9': 0,
      '10': 'channelInit'
    },
    {
      '1': 'pty_resize',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.clientipc.Msg.PtyResize',
      '9': 0,
      '10': 'ptyResize'
    },
  ],
  '3': [
    Msg_Data$json,
    Msg_PtyRequest$json,
    Msg_ShellRequest$json,
    Msg_ChannelInit$json,
    Msg_PtyResize$json
  ],
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

@$core.Deprecated('Use msgDescriptor instead')
const Msg_PtyResize$json = {
  '1': 'PtyResize',
  '2': [
    {'1': 'col_width', '3': 1, '4': 1, '5': 13, '10': 'colWidth'},
    {'1': 'row_height', '3': 2, '4': 1, '5': 13, '10': 'rowHeight'},
  ],
};

/// Descriptor for `Msg`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List msgDescriptor = $convert.base64Decode(
    'CgNNc2cSKQoEZGF0YRgBIAEoCzITLmNsaWVudGlwYy5Nc2cuRGF0YUgAUgRkYXRhEjwKC3B0eV'
    '9yZXF1ZXN0GAIgASgLMhkuY2xpZW50aXBjLk1zZy5QdHlSZXF1ZXN0SABSCnB0eVJlcXVlc3QS'
    'QgoNc2hlbGxfcmVxdWVzdBgDIAEoCzIbLmNsaWVudGlwYy5Nc2cuU2hlbGxSZXF1ZXN0SABSDH'
    'NoZWxsUmVxdWVzdBI/CgxjaGFubmVsX2luaXQYBCABKAsyGi5jbGllbnRpcGMuTXNnLkNoYW5u'
    'ZWxJbml0SABSC2NoYW5uZWxJbml0EjkKCnB0eV9yZXNpemUYBSABKAsyGC5jbGllbnRpcGMuTX'
    'NnLlB0eVJlc2l6ZUgAUglwdHlSZXNpemUaIAoERGF0YRIYCgdwYXlsb2FkGAEgASgMUgdwYXls'
    'b2FkGkgKClB0eVJlcXVlc3QSGwoJY29sX3dpZHRoGAEgASgNUghjb2xXaWR0aBIdCgpyb3dfaG'
    'VpZ2h0GAIgASgNUglyb3dIZWlnaHQaDgoMU2hlbGxSZXF1ZXN0GiwKC0NoYW5uZWxJbml0Eh0K'
    'CnNlc3Npb25faWQYASABKAlSCXNlc3Npb25JZBpHCglQdHlSZXNpemUSGwoJY29sX3dpZHRoGA'
    'EgASgNUghjb2xXaWR0aBIdCgpyb3dfaGVpZ2h0GAIgASgNUglyb3dIZWlnaHRCBgoEdHlwZQ==');

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

@$core.Deprecated('Use fileTransferStatusDescriptor instead')
const FileTransferStatus$json = {
  '1': 'FileTransferStatus',
  '2': [
    {
      '1': 'progress',
      '3': 1,
      '4': 1,
      '5': 11,
      '6': '.clientipc.FileTransferStatus.Progress',
      '9': 0,
      '10': 'progress'
    },
    {
      '1': 'completed',
      '3': 2,
      '4': 1,
      '5': 11,
      '6': '.clientipc.FileTransferStatus.Completed',
      '9': 0,
      '10': 'completed'
    },
  ],
  '3': [FileTransferStatus_Progress$json, FileTransferStatus_Completed$json],
  '8': [
    {'1': 'typ'},
  ],
};

@$core.Deprecated('Use fileTransferStatusDescriptor instead')
const FileTransferStatus_Progress$json = {
  '1': 'Progress',
  '2': [
    {'1': 'bytes_read', '3': 1, '4': 1, '5': 5, '10': 'bytesRead'},
  ],
};

@$core.Deprecated('Use fileTransferStatusDescriptor instead')
const FileTransferStatus_Completed$json = {
  '1': 'Completed',
};

/// Descriptor for `FileTransferStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List fileTransferStatusDescriptor = $convert.base64Decode(
    'ChJGaWxlVHJhbnNmZXJTdGF0dXMSRAoIcHJvZ3Jlc3MYASABKAsyJi5jbGllbnRpcGMuRmlsZV'
    'RyYW5zZmVyU3RhdHVzLlByb2dyZXNzSABSCHByb2dyZXNzEkcKCWNvbXBsZXRlZBgCIAEoCzIn'
    'LmNsaWVudGlwYy5GaWxlVHJhbnNmZXJTdGF0dXMuQ29tcGxldGVkSABSCWNvbXBsZXRlZBopCg'
    'hQcm9ncmVzcxIdCgpieXRlc19yZWFkGAEgASgFUglieXRlc1JlYWQaCwoJQ29tcGxldGVkQgUK'
    'A3R5cA==');

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
final $typed_data.Uint8List fileReadResponseDescriptor = $convert
    .base64Decode('ChBGaWxlUmVhZFJlc3BvbnNlEhIKBGRhdGEYASABKAxSBGRhdGE=');

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
final $typed_data.Uint8List dirMetadataDescriptor =
    $convert.base64Decode('CgtEaXJNZXRhZGF0YRISCgRwYXRoGAEgASgJUgRwYXRo');

@$core.Deprecated('Use fileListDescriptor instead')
const FileList$json = {
  '1': 'FileList',
  '2': [
    {
      '1': 'files',
      '3': 1,
      '4': 3,
      '5': 11,
      '6': '.clientipc.FileData',
      '10': 'files'
    },
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
final $typed_data.Uint8List ptyRequestResponseDescriptor =
    $convert.base64Decode(
        'ChJQdHlSZXF1ZXN0UmVzcG9uc2USHQoKY2hhbm5lbF9pZBgBIAEoCVIJY2hhbm5lbElk');

@$core.Deprecated('Use genKeysRequestDescriptor instead')
const GenKeysRequest$json = {
  '1': 'GenKeysRequest',
};

/// Descriptor for `GenKeysRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List genKeysRequestDescriptor =
    $convert.base64Decode('Cg5HZW5LZXlzUmVxdWVzdA==');

@$core.Deprecated('Use genKeysResponseDescriptor instead')
const GenKeysResponse$json = {
  '1': 'GenKeysResponse',
};

/// Descriptor for `GenKeysResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List genKeysResponseDescriptor =
    $convert.base64Decode('Cg9HZW5LZXlzUmVzcG9uc2U=');

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
final $typed_data.Uint8List streamResponseDescriptor =
    $convert.base64Decode('Cg5TdHJlYW1SZXNwb25zZRISCgRkYXRhGAEgASgMUgRkYXRh');

@$core.Deprecated('Use newSessionRequestDescriptor instead')
const NewSessionRequest$json = {
  '1': 'NewSessionRequest',
  '2': [
    {'1': 'private_key', '3': 3, '4': 1, '5': 9, '10': 'privateKey'},
    {'1': 'known_hosts_path', '3': 4, '4': 1, '5': 9, '10': 'knownHostsPath'},
    {
      '1': 'session_data',
      '3': 5,
      '4': 1,
      '5': 11,
      '6': '.clientipc.SessionData',
      '10': 'sessionData'
    },
  ],
};

/// Descriptor for `NewSessionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSessionRequestDescriptor = $convert.base64Decode(
    'ChFOZXdTZXNzaW9uUmVxdWVzdBIfCgtwcml2YXRlX2tleRgDIAEoCVIKcHJpdmF0ZUtleRIoCh'
    'Brbm93bl9ob3N0c19wYXRoGAQgASgJUg5rbm93bkhvc3RzUGF0aBI5CgxzZXNzaW9uX2RhdGEY'
    'BSABKAsyFi5jbGllbnRpcGMuU2Vzc2lvbkRhdGFSC3Nlc3Npb25EYXRh');

@$core.Deprecated('Use newSessionResponseDescriptor instead')
const NewSessionResponse$json = {
  '1': 'NewSessionResponse',
  '2': [
    {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `NewSessionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newSessionResponseDescriptor =
    $convert.base64Decode(
        'ChJOZXdTZXNzaW9uUmVzcG9uc2USHQoKc2Vzc2lvbl9pZBgBIAEoCVIJc2Vzc2lvbklk');

@$core.Deprecated('Use newConnectionRequestDescriptor instead')
const NewConnectionRequest$json = {
  '1': 'NewConnectionRequest',
  '2': [
    {'1': 'coordinator_url', '3': 1, '4': 1, '5': 9, '10': 'coordinatorUrl'},
    {'1': 'target_id', '3': 2, '4': 1, '5': 9, '10': 'targetId'},
    {
      '1': 'own_ipv6',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'ownIpv6',
      '17': true
    },
  ],
  '8': [
    {'1': '_own_ipv6'},
  ],
};

/// Descriptor for `NewConnectionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newConnectionRequestDescriptor = $convert.base64Decode(
    'ChROZXdDb25uZWN0aW9uUmVxdWVzdBInCg9jb29yZGluYXRvcl91cmwYASABKAlSDmNvb3JkaW'
    '5hdG9yVXJsEhsKCXRhcmdldF9pZBgCIAEoCVIIdGFyZ2V0SWQSHgoIb3duX2lwdjYYAyABKAlI'
    'AFIHb3duSXB2NogBAUILCglfb3duX2lwdjY=');

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
