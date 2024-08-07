//
//  Generated code. Do not modify.
//  source: client_ipc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

class SessionMap extends $pb.GeneratedMessage {
  factory SessionMap({
    $core.Map<$core.String, SessionData>? map,
    $core.Map<$core.String, DeviceStatus>? parents,
  }) {
    final $result = create();
    if (map != null) {
      $result.map.addAll(map);
    }
    if (parents != null) {
      $result.parents.addAll(parents);
    }
    return $result;
  }
  SessionMap._() : super();
  factory SessionMap.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionMap.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionMap',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..m<$core.String, SessionData>(1, _omitFieldNames ? '' : 'map',
        entryClassName: 'SessionMap.MapEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OM,
        valueCreator: SessionData.create,
        valueDefaultOrMaker: SessionData.getDefault,
        packageName: const $pb.PackageName('clientipc'))
    ..m<$core.String, DeviceStatus>(2, _omitFieldNames ? '' : 'parents',
        entryClassName: 'SessionMap.ParentsEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OM,
        valueCreator: DeviceStatus.create,
        valueDefaultOrMaker: DeviceStatus.getDefault,
        packageName: const $pb.PackageName('clientipc'))
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionMap clone() => SessionMap()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionMap copyWith(void Function(SessionMap) updates) =>
      super.copyWith((message) => updates(message as SessionMap)) as SessionMap;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionMap create() => SessionMap._();
  SessionMap createEmptyInstance() => create();
  static $pb.PbList<SessionMap> createRepeated() => $pb.PbList<SessionMap>();
  @$core.pragma('dart2js:noInline')
  static SessionMap getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionMap>(create);
  static SessionMap? _defaultInstance;

  /// Device, session data
  @$pb.TagNumber(1)
  $core.Map<$core.String, SessionData> get map => $_getMap(0);

  /// Device, Device status
  @$pb.TagNumber(2)
  $core.Map<$core.String, DeviceStatus> get parents => $_getMap(1);
}

class DeviceStatus extends $pb.GeneratedMessage {
  factory DeviceStatus({
    $core.bool? connected,
  }) {
    final $result = create();
    if (connected != null) {
      $result.connected = connected;
    }
    return $result;
  }
  DeviceStatus._() : super();
  factory DeviceStatus.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeviceStatus.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DeviceStatus',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'connected')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeviceStatus clone() => DeviceStatus()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeviceStatus copyWith(void Function(DeviceStatus) updates) =>
      super.copyWith((message) => updates(message as DeviceStatus))
          as DeviceStatus;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DeviceStatus create() => DeviceStatus._();
  DeviceStatus createEmptyInstance() => create();
  static $pb.PbList<DeviceStatus> createRepeated() =>
      $pb.PbList<DeviceStatus>();
  @$core.pragma('dart2js:noInline')
  static DeviceStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeviceStatus>(create);
  static DeviceStatus? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get connected => $_getBF(0);
  @$pb.TagNumber(1)
  set connected($core.bool v) {
    $_setBool(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasConnected() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnected() => clearField(1);
}

class SessionRequest extends $pb.GeneratedMessage {
  factory SessionRequest({
    $core.String? parent,
  }) {
    final $result = create();
    if (parent != null) {
      $result.parent = parent;
    }
    return $result;
  }
  SessionRequest._() : super();
  factory SessionRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'parent')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionRequest clone() => SessionRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionRequest copyWith(void Function(SessionRequest) updates) =>
      super.copyWith((message) => updates(message as SessionRequest))
          as SessionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionRequest create() => SessionRequest._();
  SessionRequest createEmptyInstance() => create();
  static $pb.PbList<SessionRequest> createRepeated() =>
      $pb.PbList<SessionRequest>();
  @$core.pragma('dart2js:noInline')
  static SessionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionRequest>(create);
  static SessionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get parent => $_getSZ(0);
  @$pb.TagNumber(1)
  set parent($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasParent() => $_has(0);
  @$pb.TagNumber(1)
  void clearParent() => clearField(1);
}

class SessionData_PTYSession extends $pb.GeneratedMessage {
  factory SessionData_PTYSession() => create();
  SessionData_PTYSession._() : super();
  factory SessionData_PTYSession.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionData_PTYSession.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionData.PTYSession',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionData_PTYSession clone() =>
      SessionData_PTYSession()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionData_PTYSession copyWith(
          void Function(SessionData_PTYSession) updates) =>
      super.copyWith((message) => updates(message as SessionData_PTYSession))
          as SessionData_PTYSession;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionData_PTYSession create() => SessionData_PTYSession._();
  SessionData_PTYSession createEmptyInstance() => create();
  static $pb.PbList<SessionData_PTYSession> createRepeated() =>
      $pb.PbList<SessionData_PTYSession>();
  @$core.pragma('dart2js:noInline')
  static SessionData_PTYSession getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionData_PTYSession>(create);
  static SessionData_PTYSession? _defaultInstance;
}

class SessionData_SFTPSession extends $pb.GeneratedMessage {
  factory SessionData_SFTPSession() => create();
  SessionData_SFTPSession._() : super();
  factory SessionData_SFTPSession.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionData_SFTPSession.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionData.SFTPSession',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionData_SFTPSession clone() =>
      SessionData_SFTPSession()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionData_SFTPSession copyWith(
          void Function(SessionData_SFTPSession) updates) =>
      super.copyWith((message) => updates(message as SessionData_SFTPSession))
          as SessionData_SFTPSession;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionData_SFTPSession create() => SessionData_SFTPSession._();
  SessionData_SFTPSession createEmptyInstance() => create();
  static $pb.PbList<SessionData_SFTPSession> createRepeated() =>
      $pb.PbList<SessionData_SFTPSession>();
  @$core.pragma('dart2js:noInline')
  static SessionData_SFTPSession getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionData_SFTPSession>(create);
  static SessionData_SFTPSession? _defaultInstance;
}

class SessionData_LPFSession extends $pb.GeneratedMessage {
  factory SessionData_LPFSession({
    $core.String? localHost,
    $core.int? localPort,
    $core.String? remoteHost,
    $core.int? remotePort,
  }) {
    final $result = create();
    if (localHost != null) {
      $result.localHost = localHost;
    }
    if (localPort != null) {
      $result.localPort = localPort;
    }
    if (remoteHost != null) {
      $result.remoteHost = remoteHost;
    }
    if (remotePort != null) {
      $result.remotePort = remotePort;
    }
    return $result;
  }
  SessionData_LPFSession._() : super();
  factory SessionData_LPFSession.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionData_LPFSession.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionData.LPFSession',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'localHost')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'localPort', $pb.PbFieldType.OU3)
    ..aOS(3, _omitFieldNames ? '' : 'remoteHost')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'remotePort', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionData_LPFSession clone() =>
      SessionData_LPFSession()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionData_LPFSession copyWith(
          void Function(SessionData_LPFSession) updates) =>
      super.copyWith((message) => updates(message as SessionData_LPFSession))
          as SessionData_LPFSession;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionData_LPFSession create() => SessionData_LPFSession._();
  SessionData_LPFSession createEmptyInstance() => create();
  static $pb.PbList<SessionData_LPFSession> createRepeated() =>
      $pb.PbList<SessionData_LPFSession>();
  @$core.pragma('dart2js:noInline')
  static SessionData_LPFSession getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionData_LPFSession>(create);
  static SessionData_LPFSession? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get localHost => $_getSZ(0);
  @$pb.TagNumber(1)
  set localHost($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasLocalHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearLocalHost() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get localPort => $_getIZ(1);
  @$pb.TagNumber(2)
  set localPort($core.int v) {
    $_setUnsignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasLocalPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearLocalPort() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get remoteHost => $_getSZ(2);
  @$pb.TagNumber(3)
  set remoteHost($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasRemoteHost() => $_has(2);
  @$pb.TagNumber(3)
  void clearRemoteHost() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get remotePort => $_getIZ(3);
  @$pb.TagNumber(4)
  set remotePort($core.int v) {
    $_setUnsignedInt32(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasRemotePort() => $_has(3);
  @$pb.TagNumber(4)
  void clearRemotePort() => clearField(4);
}

enum SessionData_Kind { pty, sftp, lpf, notSet }

class SessionData extends $pb.GeneratedMessage {
  factory SessionData({
    SessionData_PTYSession? pty,
    SessionData_SFTPSession? sftp,
    SessionData_LPFSession? lpf,
    $core.String? sessionId,
    $core.String? username,
    $core.String? deviceId,
  }) {
    final $result = create();
    if (pty != null) {
      $result.pty = pty;
    }
    if (sftp != null) {
      $result.sftp = sftp;
    }
    if (lpf != null) {
      $result.lpf = lpf;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    if (username != null) {
      $result.username = username;
    }
    if (deviceId != null) {
      $result.deviceId = deviceId;
    }
    return $result;
  }
  SessionData._() : super();
  factory SessionData.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SessionData.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, SessionData_Kind> _SessionData_KindByTag = {
    1: SessionData_Kind.pty,
    2: SessionData_Kind.sftp,
    3: SessionData_Kind.lpf,
    0: SessionData_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SessionData',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3])
    ..aOM<SessionData_PTYSession>(1, _omitFieldNames ? '' : 'pty',
        subBuilder: SessionData_PTYSession.create)
    ..aOM<SessionData_SFTPSession>(2, _omitFieldNames ? '' : 'sftp',
        subBuilder: SessionData_SFTPSession.create)
    ..aOM<SessionData_LPFSession>(3, _omitFieldNames ? '' : 'lpf',
        subBuilder: SessionData_LPFSession.create)
    ..aOS(4, _omitFieldNames ? '' : 'sessionId')
    ..aOS(5, _omitFieldNames ? '' : 'username')
    ..aOS(6, _omitFieldNames ? '' : 'deviceId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SessionData clone() => SessionData()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SessionData copyWith(void Function(SessionData) updates) =>
      super.copyWith((message) => updates(message as SessionData))
          as SessionData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SessionData create() => SessionData._();
  SessionData createEmptyInstance() => create();
  static $pb.PbList<SessionData> createRepeated() => $pb.PbList<SessionData>();
  @$core.pragma('dart2js:noInline')
  static SessionData getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SessionData>(create);
  static SessionData? _defaultInstance;

  SessionData_Kind whichKind() => _SessionData_KindByTag[$_whichOneof(0)]!;
  void clearKind() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  SessionData_PTYSession get pty => $_getN(0);
  @$pb.TagNumber(1)
  set pty(SessionData_PTYSession v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPty() => $_has(0);
  @$pb.TagNumber(1)
  void clearPty() => clearField(1);
  @$pb.TagNumber(1)
  SessionData_PTYSession ensurePty() => $_ensure(0);

  @$pb.TagNumber(2)
  SessionData_SFTPSession get sftp => $_getN(1);
  @$pb.TagNumber(2)
  set sftp(SessionData_SFTPSession v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSftp() => $_has(1);
  @$pb.TagNumber(2)
  void clearSftp() => clearField(2);
  @$pb.TagNumber(2)
  SessionData_SFTPSession ensureSftp() => $_ensure(1);

  @$pb.TagNumber(3)
  SessionData_LPFSession get lpf => $_getN(2);
  @$pb.TagNumber(3)
  set lpf(SessionData_LPFSession v) {
    setField(3, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasLpf() => $_has(2);
  @$pb.TagNumber(3)
  void clearLpf() => clearField(3);
  @$pb.TagNumber(3)
  SessionData_LPFSession ensureLpf() => $_ensure(2);

  /// ID of the server
  @$pb.TagNumber(4)
  $core.String get sessionId => $_getSZ(3);
  @$pb.TagNumber(4)
  set sessionId($core.String v) {
    $_setString(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasSessionId() => $_has(3);
  @$pb.TagNumber(4)
  void clearSessionId() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get username => $_getSZ(4);
  @$pb.TagNumber(5)
  set username($core.String v) {
    $_setString(4, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasUsername() => $_has(4);
  @$pb.TagNumber(5)
  void clearUsername() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get deviceId => $_getSZ(5);
  @$pb.TagNumber(6)
  set deviceId($core.String v) {
    $_setString(5, v);
  }

  @$pb.TagNumber(6)
  $core.bool hasDeviceId() => $_has(5);
  @$pb.TagNumber(6)
  void clearDeviceId() => clearField(6);
}

class LocalPortForwardResponse extends $pb.GeneratedMessage {
  factory LocalPortForwardResponse() => create();
  LocalPortForwardResponse._() : super();
  factory LocalPortForwardResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory LocalPortForwardResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LocalPortForwardResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  LocalPortForwardResponse clone() =>
      LocalPortForwardResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  LocalPortForwardResponse copyWith(
          void Function(LocalPortForwardResponse) updates) =>
      super.copyWith((message) => updates(message as LocalPortForwardResponse))
          as LocalPortForwardResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocalPortForwardResponse create() => LocalPortForwardResponse._();
  LocalPortForwardResponse createEmptyInstance() => create();
  static $pb.PbList<LocalPortForwardResponse> createRepeated() =>
      $pb.PbList<LocalPortForwardResponse>();
  @$core.pragma('dart2js:noInline')
  static LocalPortForwardResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LocalPortForwardResponse>(create);
  static LocalPortForwardResponse? _defaultInstance;
}

class LocalPortForwardRequest extends $pb.GeneratedMessage {
  factory LocalPortForwardRequest({
    $core.String? localHost,
    $core.int? localPort,
    $core.String? remoteHost,
    $core.int? remotePort,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (localHost != null) {
      $result.localHost = localHost;
    }
    if (localPort != null) {
      $result.localPort = localPort;
    }
    if (remoteHost != null) {
      $result.remoteHost = remoteHost;
    }
    if (remotePort != null) {
      $result.remotePort = remotePort;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  LocalPortForwardRequest._() : super();
  factory LocalPortForwardRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory LocalPortForwardRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'LocalPortForwardRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'localHost')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'localPort', $pb.PbFieldType.OU3)
    ..aOS(3, _omitFieldNames ? '' : 'remoteHost')
    ..a<$core.int>(4, _omitFieldNames ? '' : 'remotePort', $pb.PbFieldType.OU3)
    ..aOS(5, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  LocalPortForwardRequest clone() =>
      LocalPortForwardRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  LocalPortForwardRequest copyWith(
          void Function(LocalPortForwardRequest) updates) =>
      super.copyWith((message) => updates(message as LocalPortForwardRequest))
          as LocalPortForwardRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static LocalPortForwardRequest create() => LocalPortForwardRequest._();
  LocalPortForwardRequest createEmptyInstance() => create();
  static $pb.PbList<LocalPortForwardRequest> createRepeated() =>
      $pb.PbList<LocalPortForwardRequest>();
  @$core.pragma('dart2js:noInline')
  static LocalPortForwardRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<LocalPortForwardRequest>(create);
  static LocalPortForwardRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get localHost => $_getSZ(0);
  @$pb.TagNumber(1)
  set localHost($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasLocalHost() => $_has(0);
  @$pb.TagNumber(1)
  void clearLocalHost() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get localPort => $_getIZ(1);
  @$pb.TagNumber(2)
  set localPort($core.int v) {
    $_setUnsignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasLocalPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearLocalPort() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get remoteHost => $_getSZ(2);
  @$pb.TagNumber(3)
  set remoteHost($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasRemoteHost() => $_has(2);
  @$pb.TagNumber(3)
  void clearRemoteHost() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get remotePort => $_getIZ(3);
  @$pb.TagNumber(4)
  set remotePort($core.int v) {
    $_setUnsignedInt32(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasRemotePort() => $_has(3);
  @$pb.TagNumber(4)
  void clearRemotePort() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get sessionId => $_getSZ(4);
  @$pb.TagNumber(5)
  set sessionId($core.String v) {
    $_setString(4, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasSessionId() => $_has(4);
  @$pb.TagNumber(5)
  void clearSessionId() => clearField(5);
}

class FileDeleteRequest extends $pb.GeneratedMessage {
  factory FileDeleteRequest({
    $core.Iterable<$core.String>? files,
  }) {
    final $result = create();
    if (files != null) {
      $result.files.addAll(files);
    }
    return $result;
  }
  FileDeleteRequest._() : super();
  factory FileDeleteRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileDeleteRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileDeleteRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'files')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileDeleteRequest clone() => FileDeleteRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileDeleteRequest copyWith(void Function(FileDeleteRequest) updates) =>
      super.copyWith((message) => updates(message as FileDeleteRequest))
          as FileDeleteRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileDeleteRequest create() => FileDeleteRequest._();
  FileDeleteRequest createEmptyInstance() => create();
  static $pb.PbList<FileDeleteRequest> createRepeated() =>
      $pb.PbList<FileDeleteRequest>();
  @$core.pragma('dart2js:noInline')
  static FileDeleteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileDeleteRequest>(create);
  static FileDeleteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get files => $_getList(0);
}

class GetKeyRequest extends $pb.GeneratedMessage {
  factory GetKeyRequest() => create();
  GetKeyRequest._() : super();
  factory GetKeyRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetKeyRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetKeyRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetKeyRequest clone() => GetKeyRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetKeyRequest copyWith(void Function(GetKeyRequest) updates) =>
      super.copyWith((message) => updates(message as GetKeyRequest))
          as GetKeyRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetKeyRequest create() => GetKeyRequest._();
  GetKeyRequest createEmptyInstance() => create();
  static $pb.PbList<GetKeyRequest> createRepeated() =>
      $pb.PbList<GetKeyRequest>();
  @$core.pragma('dart2js:noInline')
  static GetKeyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetKeyRequest>(create);
  static GetKeyRequest? _defaultInstance;
}

class PublicKey extends $pb.GeneratedMessage {
  factory PublicKey({
    $core.String? key,
  }) {
    final $result = create();
    if (key != null) {
      $result.key = key;
    }
    return $result;
  }
  PublicKey._() : super();
  factory PublicKey.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory PublicKey.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PublicKey',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'key')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  PublicKey clone() => PublicKey()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  PublicKey copyWith(void Function(PublicKey) updates) =>
      super.copyWith((message) => updates(message as PublicKey)) as PublicKey;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PublicKey create() => PublicKey._();
  PublicKey createEmptyInstance() => create();
  static $pb.PbList<PublicKey> createRepeated() => $pb.PbList<PublicKey>();
  @$core.pragma('dart2js:noInline')
  static PublicKey getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PublicKey>(create);
  static PublicKey? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get key => $_getSZ(0);
  @$pb.TagNumber(1)
  set key($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => clearField(1);
}

class InitData extends $pb.GeneratedMessage {
  factory InitData({
    $core.String? dataFolderPath,
  }) {
    final $result = create();
    if (dataFolderPath != null) {
      $result.dataFolderPath = dataFolderPath;
    }
    return $result;
  }
  InitData._() : super();
  factory InitData.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory InitData.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InitData',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'dataFolderPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  InitData clone() => InitData()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  InitData copyWith(void Function(InitData) updates) =>
      super.copyWith((message) => updates(message as InitData)) as InitData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InitData create() => InitData._();
  InitData createEmptyInstance() => create();
  static $pb.PbList<InitData> createRepeated() => $pb.PbList<InitData>();
  @$core.pragma('dart2js:noInline')
  static InitData getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InitData>(create);
  static InitData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get dataFolderPath => $_getSZ(0);
  @$pb.TagNumber(1)
  set dataFolderPath($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasDataFolderPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearDataFolderPath() => clearField(1);
}

class InitResponse extends $pb.GeneratedMessage {
  factory InitResponse() => create();
  InitResponse._() : super();
  factory InitResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory InitResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'InitResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  InitResponse clone() => InitResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  InitResponse copyWith(void Function(InitResponse) updates) =>
      super.copyWith((message) => updates(message as InitResponse))
          as InitResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InitResponse create() => InitResponse._();
  InitResponse createEmptyInstance() => create();
  static $pb.PbList<InitResponse> createRepeated() =>
      $pb.PbList<InitResponse>();
  @$core.pragma('dart2js:noInline')
  static InitResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<InitResponse>(create);
  static InitResponse? _defaultInstance;
}

class GetSaveDataRequest extends $pb.GeneratedMessage {
  factory GetSaveDataRequest() => create();
  GetSaveDataRequest._() : super();
  factory GetSaveDataRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetSaveDataRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GetSaveDataRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetSaveDataRequest clone() => GetSaveDataRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetSaveDataRequest copyWith(void Function(GetSaveDataRequest) updates) =>
      super.copyWith((message) => updates(message as GetSaveDataRequest))
          as GetSaveDataRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetSaveDataRequest create() => GetSaveDataRequest._();
  GetSaveDataRequest createEmptyInstance() => create();
  static $pb.PbList<GetSaveDataRequest> createRepeated() =>
      $pb.PbList<GetSaveDataRequest>();
  @$core.pragma('dart2js:noInline')
  static GetSaveDataRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetSaveDataRequest>(create);
  static GetSaveDataRequest? _defaultInstance;
}

class SettingsRequest extends $pb.GeneratedMessage {
  factory SettingsRequest() => create();
  SettingsRequest._() : super();
  factory SettingsRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SettingsRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SettingsRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SettingsRequest clone() => SettingsRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SettingsRequest copyWith(void Function(SettingsRequest) updates) =>
      super.copyWith((message) => updates(message as SettingsRequest))
          as SettingsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SettingsRequest create() => SettingsRequest._();
  SettingsRequest createEmptyInstance() => create();
  static $pb.PbList<SettingsRequest> createRepeated() =>
      $pb.PbList<SettingsRequest>();
  @$core.pragma('dart2js:noInline')
  static SettingsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SettingsRequest>(create);
  static SettingsRequest? _defaultInstance;
}

class Settings extends $pb.GeneratedMessage {
  factory Settings({
    $core.String? coordinatorUrl,
    $core.String? deviceId,
  }) {
    final $result = create();
    if (coordinatorUrl != null) {
      $result.coordinatorUrl = coordinatorUrl;
    }
    if (deviceId != null) {
      $result.deviceId = deviceId;
    }
    return $result;
  }
  Settings._() : super();
  factory Settings.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Settings.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Settings',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'coordinatorUrl')
    ..aOS(2, _omitFieldNames ? '' : 'deviceId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Settings clone() => Settings()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Settings copyWith(void Function(Settings) updates) =>
      super.copyWith((message) => updates(message as Settings)) as Settings;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Settings create() => Settings._();
  Settings createEmptyInstance() => create();
  static $pb.PbList<Settings> createRepeated() => $pb.PbList<Settings>();
  @$core.pragma('dart2js:noInline')
  static Settings getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Settings>(create);
  static Settings? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get coordinatorUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set coordinatorUrl($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasCoordinatorUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearCoordinatorUrl() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get deviceId => $_getSZ(1);
  @$pb.TagNumber(2)
  set deviceId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasDeviceId() => $_has(1);
  @$pb.TagNumber(2)
  void clearDeviceId() => clearField(2);
}

class UserData extends $pb.GeneratedMessage {
  factory UserData({
    $core.Iterable<$core.String>? usedDeviceIds,
    $core.Map<$core.String, SessionData>? savedSessions,
  }) {
    final $result = create();
    if (usedDeviceIds != null) {
      $result.usedDeviceIds.addAll(usedDeviceIds);
    }
    if (savedSessions != null) {
      $result.savedSessions.addAll(savedSessions);
    }
    return $result;
  }
  UserData._() : super();
  factory UserData.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory UserData.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'UserData',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..pPS(1, _omitFieldNames ? '' : 'usedDeviceIds')
    ..m<$core.String, SessionData>(2, _omitFieldNames ? '' : 'savedSessions',
        entryClassName: 'UserData.SavedSessionsEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OM,
        valueCreator: SessionData.create,
        valueDefaultOrMaker: SessionData.getDefault,
        packageName: const $pb.PackageName('clientipc'))
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  UserData clone() => UserData()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  UserData copyWith(void Function(UserData) updates) =>
      super.copyWith((message) => updates(message as UserData)) as UserData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static UserData create() => UserData._();
  UserData createEmptyInstance() => create();
  static $pb.PbList<UserData> createRepeated() => $pb.PbList<UserData>();
  @$core.pragma('dart2js:noInline')
  static UserData getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserData>(create);
  static UserData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.String> get usedDeviceIds => $_getList(0);

  @$pb.TagNumber(2)
  $core.Map<$core.String, SessionData> get savedSessions => $_getMap(1);
}

class EmptyValue extends $pb.GeneratedMessage {
  factory EmptyValue() => create();
  EmptyValue._() : super();
  factory EmptyValue.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory EmptyValue.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'EmptyValue',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  EmptyValue clone() => EmptyValue()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  EmptyValue copyWith(void Function(EmptyValue) updates) =>
      super.copyWith((message) => updates(message as EmptyValue)) as EmptyValue;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static EmptyValue create() => EmptyValue._();
  EmptyValue createEmptyInstance() => create();
  static $pb.PbList<EmptyValue> createRepeated() => $pb.PbList<EmptyValue>();
  @$core.pragma('dart2js:noInline')
  static EmptyValue getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EmptyValue>(create);
  static EmptyValue? _defaultInstance;
}

enum Value_Kind { stringValue, intValue, jsonValue, listValue, empty, notSet }

class Value extends $pb.GeneratedMessage {
  factory Value({
    $core.String? stringValue,
    $core.int? intValue,
    MapValue? jsonValue,
    ListValue? listValue,
    EmptyValue? empty,
  }) {
    final $result = create();
    if (stringValue != null) {
      $result.stringValue = stringValue;
    }
    if (intValue != null) {
      $result.intValue = intValue;
    }
    if (jsonValue != null) {
      $result.jsonValue = jsonValue;
    }
    if (listValue != null) {
      $result.listValue = listValue;
    }
    if (empty != null) {
      $result.empty = empty;
    }
    return $result;
  }
  Value._() : super();
  factory Value.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Value.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Value_Kind> _Value_KindByTag = {
    1: Value_Kind.stringValue,
    2: Value_Kind.intValue,
    3: Value_Kind.jsonValue,
    4: Value_Kind.listValue,
    5: Value_Kind.empty,
    0: Value_Kind.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Value',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5])
    ..aOS(1, _omitFieldNames ? '' : 'stringValue')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'intValue', $pb.PbFieldType.O3)
    ..aOM<MapValue>(3, _omitFieldNames ? '' : 'jsonValue',
        subBuilder: MapValue.create)
    ..aOM<ListValue>(4, _omitFieldNames ? '' : 'listValue',
        subBuilder: ListValue.create)
    ..aOM<EmptyValue>(5, _omitFieldNames ? '' : 'empty',
        subBuilder: EmptyValue.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Value clone() => Value()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Value copyWith(void Function(Value) updates) =>
      super.copyWith((message) => updates(message as Value)) as Value;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Value create() => Value._();
  Value createEmptyInstance() => create();
  static $pb.PbList<Value> createRepeated() => $pb.PbList<Value>();
  @$core.pragma('dart2js:noInline')
  static Value getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Value>(create);
  static Value? _defaultInstance;

  Value_Kind whichKind() => _Value_KindByTag[$_whichOneof(0)]!;
  void clearKind() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get stringValue => $_getSZ(0);
  @$pb.TagNumber(1)
  set stringValue($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasStringValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearStringValue() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get intValue => $_getIZ(1);
  @$pb.TagNumber(2)
  set intValue($core.int v) {
    $_setSignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasIntValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearIntValue() => clearField(2);

  @$pb.TagNumber(3)
  MapValue get jsonValue => $_getN(2);
  @$pb.TagNumber(3)
  set jsonValue(MapValue v) {
    setField(3, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasJsonValue() => $_has(2);
  @$pb.TagNumber(3)
  void clearJsonValue() => clearField(3);
  @$pb.TagNumber(3)
  MapValue ensureJsonValue() => $_ensure(2);

  @$pb.TagNumber(4)
  ListValue get listValue => $_getN(3);
  @$pb.TagNumber(4)
  set listValue(ListValue v) {
    setField(4, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasListValue() => $_has(3);
  @$pb.TagNumber(4)
  void clearListValue() => clearField(4);
  @$pb.TagNumber(4)
  ListValue ensureListValue() => $_ensure(3);

  @$pb.TagNumber(5)
  EmptyValue get empty => $_getN(4);
  @$pb.TagNumber(5)
  set empty(EmptyValue v) {
    setField(5, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasEmpty() => $_has(4);
  @$pb.TagNumber(5)
  void clearEmpty() => clearField(5);
  @$pb.TagNumber(5)
  EmptyValue ensureEmpty() => $_ensure(4);
}

class MapValue extends $pb.GeneratedMessage {
  factory MapValue({
    $core.Map<$core.String, Value>? map,
  }) {
    final $result = create();
    if (map != null) {
      $result.map.addAll(map);
    }
    return $result;
  }
  MapValue._() : super();
  factory MapValue.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory MapValue.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'MapValue',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..m<$core.String, Value>(1, _omitFieldNames ? '' : 'map',
        entryClassName: 'MapValue.MapEntry',
        keyFieldType: $pb.PbFieldType.OS,
        valueFieldType: $pb.PbFieldType.OM,
        valueCreator: Value.create,
        valueDefaultOrMaker: Value.getDefault,
        packageName: const $pb.PackageName('clientipc'))
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  MapValue clone() => MapValue()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  MapValue copyWith(void Function(MapValue) updates) =>
      super.copyWith((message) => updates(message as MapValue)) as MapValue;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static MapValue create() => MapValue._();
  MapValue createEmptyInstance() => create();
  static $pb.PbList<MapValue> createRepeated() => $pb.PbList<MapValue>();
  @$core.pragma('dart2js:noInline')
  static MapValue getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MapValue>(create);
  static MapValue? _defaultInstance;

  @$pb.TagNumber(1)
  $core.Map<$core.String, Value> get map => $_getMap(0);
}

class ListValue extends $pb.GeneratedMessage {
  factory ListValue({
    $core.Iterable<Value>? list,
  }) {
    final $result = create();
    if (list != null) {
      $result.list.addAll(list);
    }
    return $result;
  }
  ListValue._() : super();
  factory ListValue.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListValue.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListValue',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..pc<Value>(1, _omitFieldNames ? '' : 'list', $pb.PbFieldType.PM,
        subBuilder: Value.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListValue clone() => ListValue()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListValue copyWith(void Function(ListValue) updates) =>
      super.copyWith((message) => updates(message as ListValue)) as ListValue;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListValue create() => ListValue._();
  ListValue createEmptyInstance() => create();
  static $pb.PbList<ListValue> createRepeated() => $pb.PbList<ListValue>();
  @$core.pragma('dart2js:noInline')
  static ListValue getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListValue>(create);
  static ListValue? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Value> get list => $_getList(0);
}

class SftpRequest extends $pb.GeneratedMessage {
  factory SftpRequest({
    $core.String? sessionId,
  }) {
    final $result = create();
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  SftpRequest._() : super();
  factory SftpRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SftpRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SftpRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SftpRequest clone() => SftpRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SftpRequest copyWith(void Function(SftpRequest) updates) =>
      super.copyWith((message) => updates(message as SftpRequest))
          as SftpRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SftpRequest create() => SftpRequest._();
  SftpRequest createEmptyInstance() => create();
  static $pb.PbList<SftpRequest> createRepeated() => $pb.PbList<SftpRequest>();
  @$core.pragma('dart2js:noInline')
  static SftpRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SftpRequest>(create);
  static SftpRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => clearField(1);
}

class SftpRequestResponse extends $pb.GeneratedMessage {
  factory SftpRequestResponse({
    $core.String? channelId,
  }) {
    final $result = create();
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  SftpRequestResponse._() : super();
  factory SftpRequestResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory SftpRequestResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SftpRequestResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'channelId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  SftpRequestResponse clone() => SftpRequestResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  SftpRequestResponse copyWith(void Function(SftpRequestResponse) updates) =>
      super.copyWith((message) => updates(message as SftpRequestResponse))
          as SftpRequestResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SftpRequestResponse create() => SftpRequestResponse._();
  SftpRequestResponse createEmptyInstance() => create();
  static $pb.PbList<SftpRequestResponse> createRepeated() =>
      $pb.PbList<SftpRequestResponse>();
  @$core.pragma('dart2js:noInline')
  static SftpRequestResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SftpRequestResponse>(create);
  static SftpRequestResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get channelId => $_getSZ(0);
  @$pb.TagNumber(1)
  set channelId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class Msg_Data extends $pb.GeneratedMessage {
  factory Msg_Data({
    $core.List<$core.int>? payload,
  }) {
    final $result = create();
    if (payload != null) {
      $result.payload = payload;
    }
    return $result;
  }
  Msg_Data._() : super();
  factory Msg_Data.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg_Data.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg.Data',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg_Data clone() => Msg_Data()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg_Data copyWith(void Function(Msg_Data) updates) =>
      super.copyWith((message) => updates(message as Msg_Data)) as Msg_Data;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_Data create() => Msg_Data._();
  Msg_Data createEmptyInstance() => create();
  static $pb.PbList<Msg_Data> createRepeated() => $pb.PbList<Msg_Data>();
  @$core.pragma('dart2js:noInline')
  static Msg_Data getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg_Data>(create);
  static Msg_Data? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get payload => $_getN(0);
  @$pb.TagNumber(1)
  set payload($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPayload() => $_has(0);
  @$pb.TagNumber(1)
  void clearPayload() => clearField(1);
}

class Msg_PtyRequest extends $pb.GeneratedMessage {
  factory Msg_PtyRequest({
    $core.int? colWidth,
    $core.int? rowHeight,
  }) {
    final $result = create();
    if (colWidth != null) {
      $result.colWidth = colWidth;
    }
    if (rowHeight != null) {
      $result.rowHeight = rowHeight;
    }
    return $result;
  }
  Msg_PtyRequest._() : super();
  factory Msg_PtyRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg_PtyRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg.PtyRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'colWidth', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rowHeight', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg_PtyRequest clone() => Msg_PtyRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg_PtyRequest copyWith(void Function(Msg_PtyRequest) updates) =>
      super.copyWith((message) => updates(message as Msg_PtyRequest))
          as Msg_PtyRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_PtyRequest create() => Msg_PtyRequest._();
  Msg_PtyRequest createEmptyInstance() => create();
  static $pb.PbList<Msg_PtyRequest> createRepeated() =>
      $pb.PbList<Msg_PtyRequest>();
  @$core.pragma('dart2js:noInline')
  static Msg_PtyRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Msg_PtyRequest>(create);
  static Msg_PtyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get colWidth => $_getIZ(0);
  @$pb.TagNumber(1)
  set colWidth($core.int v) {
    $_setUnsignedInt32(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasColWidth() => $_has(0);
  @$pb.TagNumber(1)
  void clearColWidth() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rowHeight => $_getIZ(1);
  @$pb.TagNumber(2)
  set rowHeight($core.int v) {
    $_setUnsignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasRowHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearRowHeight() => clearField(2);
}

class Msg_ShellRequest extends $pb.GeneratedMessage {
  factory Msg_ShellRequest() => create();
  Msg_ShellRequest._() : super();
  factory Msg_ShellRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg_ShellRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg.ShellRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg_ShellRequest clone() => Msg_ShellRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg_ShellRequest copyWith(void Function(Msg_ShellRequest) updates) =>
      super.copyWith((message) => updates(message as Msg_ShellRequest))
          as Msg_ShellRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_ShellRequest create() => Msg_ShellRequest._();
  Msg_ShellRequest createEmptyInstance() => create();
  static $pb.PbList<Msg_ShellRequest> createRepeated() =>
      $pb.PbList<Msg_ShellRequest>();
  @$core.pragma('dart2js:noInline')
  static Msg_ShellRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Msg_ShellRequest>(create);
  static Msg_ShellRequest? _defaultInstance;
}

/// Initializing this generic channel
class Msg_ChannelInit extends $pb.GeneratedMessage {
  factory Msg_ChannelInit({
    $core.String? sessionId,
  }) {
    final $result = create();
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  Msg_ChannelInit._() : super();
  factory Msg_ChannelInit.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg_ChannelInit.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg.ChannelInit',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg_ChannelInit clone() => Msg_ChannelInit()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg_ChannelInit copyWith(void Function(Msg_ChannelInit) updates) =>
      super.copyWith((message) => updates(message as Msg_ChannelInit))
          as Msg_ChannelInit;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_ChannelInit create() => Msg_ChannelInit._();
  Msg_ChannelInit createEmptyInstance() => create();
  static $pb.PbList<Msg_ChannelInit> createRepeated() =>
      $pb.PbList<Msg_ChannelInit>();
  @$core.pragma('dart2js:noInline')
  static Msg_ChannelInit getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Msg_ChannelInit>(create);
  static Msg_ChannelInit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => clearField(1);
}

class Msg_PtyResize extends $pb.GeneratedMessage {
  factory Msg_PtyResize({
    $core.int? colWidth,
    $core.int? rowHeight,
  }) {
    final $result = create();
    if (colWidth != null) {
      $result.colWidth = colWidth;
    }
    if (rowHeight != null) {
      $result.rowHeight = rowHeight;
    }
    return $result;
  }
  Msg_PtyResize._() : super();
  factory Msg_PtyResize.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg_PtyResize.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg.PtyResize',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'colWidth', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rowHeight', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg_PtyResize clone() => Msg_PtyResize()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg_PtyResize copyWith(void Function(Msg_PtyResize) updates) =>
      super.copyWith((message) => updates(message as Msg_PtyResize))
          as Msg_PtyResize;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_PtyResize create() => Msg_PtyResize._();
  Msg_PtyResize createEmptyInstance() => create();
  static $pb.PbList<Msg_PtyResize> createRepeated() =>
      $pb.PbList<Msg_PtyResize>();
  @$core.pragma('dart2js:noInline')
  static Msg_PtyResize getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<Msg_PtyResize>(create);
  static Msg_PtyResize? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get colWidth => $_getIZ(0);
  @$pb.TagNumber(1)
  set colWidth($core.int v) {
    $_setUnsignedInt32(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasColWidth() => $_has(0);
  @$pb.TagNumber(1)
  void clearColWidth() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rowHeight => $_getIZ(1);
  @$pb.TagNumber(2)
  set rowHeight($core.int v) {
    $_setUnsignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasRowHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearRowHeight() => clearField(2);
}

enum Msg_Type { data, ptyRequest, shellRequest, channelInit, ptyResize, notSet }

/// This is mainly used for generic channels that require a continuous stream to frontend (not SFTP or Local port forward)
class Msg extends $pb.GeneratedMessage {
  factory Msg({
    Msg_Data? data,
    Msg_PtyRequest? ptyRequest,
    Msg_ShellRequest? shellRequest,
    Msg_ChannelInit? channelInit,
    Msg_PtyResize? ptyResize,
  }) {
    final $result = create();
    if (data != null) {
      $result.data = data;
    }
    if (ptyRequest != null) {
      $result.ptyRequest = ptyRequest;
    }
    if (shellRequest != null) {
      $result.shellRequest = shellRequest;
    }
    if (channelInit != null) {
      $result.channelInit = channelInit;
    }
    if (ptyResize != null) {
      $result.ptyResize = ptyResize;
    }
    return $result;
  }
  Msg._() : super();
  factory Msg.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Msg.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Msg_Type> _Msg_TypeByTag = {
    1: Msg_Type.data,
    2: Msg_Type.ptyRequest,
    3: Msg_Type.shellRequest,
    4: Msg_Type.channelInit,
    5: Msg_Type.ptyResize,
    0: Msg_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Msg',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5])
    ..aOM<Msg_Data>(1, _omitFieldNames ? '' : 'data',
        subBuilder: Msg_Data.create)
    ..aOM<Msg_PtyRequest>(2, _omitFieldNames ? '' : 'ptyRequest',
        subBuilder: Msg_PtyRequest.create)
    ..aOM<Msg_ShellRequest>(3, _omitFieldNames ? '' : 'shellRequest',
        subBuilder: Msg_ShellRequest.create)
    ..aOM<Msg_ChannelInit>(4, _omitFieldNames ? '' : 'channelInit',
        subBuilder: Msg_ChannelInit.create)
    ..aOM<Msg_PtyResize>(5, _omitFieldNames ? '' : 'ptyResize',
        subBuilder: Msg_PtyResize.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Msg clone() => Msg()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Msg copyWith(void Function(Msg) updates) =>
      super.copyWith((message) => updates(message as Msg)) as Msg;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg create() => Msg._();
  Msg createEmptyInstance() => create();
  static $pb.PbList<Msg> createRepeated() => $pb.PbList<Msg>();
  @$core.pragma('dart2js:noInline')
  static Msg getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg>(create);
  static Msg? _defaultInstance;

  Msg_Type whichType() => _Msg_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Msg_Data get data => $_getN(0);
  @$pb.TagNumber(1)
  set data(Msg_Data v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
  @$pb.TagNumber(1)
  Msg_Data ensureData() => $_ensure(0);

  @$pb.TagNumber(2)
  Msg_PtyRequest get ptyRequest => $_getN(1);
  @$pb.TagNumber(2)
  set ptyRequest(Msg_PtyRequest v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasPtyRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearPtyRequest() => clearField(2);
  @$pb.TagNumber(2)
  Msg_PtyRequest ensurePtyRequest() => $_ensure(1);

  @$pb.TagNumber(3)
  Msg_ShellRequest get shellRequest => $_getN(2);
  @$pb.TagNumber(3)
  set shellRequest(Msg_ShellRequest v) {
    setField(3, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasShellRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearShellRequest() => clearField(3);
  @$pb.TagNumber(3)
  Msg_ShellRequest ensureShellRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  Msg_ChannelInit get channelInit => $_getN(3);
  @$pb.TagNumber(4)
  set channelInit(Msg_ChannelInit v) {
    setField(4, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasChannelInit() => $_has(3);
  @$pb.TagNumber(4)
  void clearChannelInit() => clearField(4);
  @$pb.TagNumber(4)
  Msg_ChannelInit ensureChannelInit() => $_ensure(3);

  @$pb.TagNumber(5)
  Msg_PtyResize get ptyResize => $_getN(4);
  @$pb.TagNumber(5)
  set ptyResize(Msg_PtyResize v) {
    setField(5, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasPtyResize() => $_has(4);
  @$pb.TagNumber(5)
  void clearPtyResize() => clearField(5);
  @$pb.TagNumber(5)
  Msg_PtyResize ensurePtyResize() => $_ensure(4);
}

/// The messages a client uses to interact with the sftp session
class ListDir extends $pb.GeneratedMessage {
  factory ListDir({
    $core.String? path,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (path != null) {
      $result.path = path;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  ListDir._() : super();
  factory ListDir.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListDir.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ListDir',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListDir clone() => ListDir()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListDir copyWith(void Function(ListDir) updates) =>
      super.copyWith((message) => updates(message as ListDir)) as ListDir;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListDir create() => ListDir._();
  ListDir createEmptyInstance() => create();
  static $pb.PbList<ListDir> createRepeated() => $pb.PbList<ListDir>();
  @$core.pragma('dart2js:noInline')
  static ListDir getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListDir>(create);
  static ListDir? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionId => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => clearField(2);
}

class Path extends $pb.GeneratedMessage {
  factory Path({
    $core.String? path,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (path != null) {
      $result.path = path;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  Path._() : super();
  factory Path.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory Path.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Path',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  Path clone() => Path()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  Path copyWith(void Function(Path) updates) =>
      super.copyWith((message) => updates(message as Path)) as Path;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Path create() => Path._();
  Path createEmptyInstance() => create();
  static $pb.PbList<Path> createRepeated() => $pb.PbList<Path>();
  @$core.pragma('dart2js:noInline')
  static Path getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Path>(create);
  static Path? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionId => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => clearField(2);
}

class FileTransferRequest extends $pb.GeneratedMessage {
  factory FileTransferRequest({
    $core.String? sessionId,
    $core.String? remotePath,
    $core.String? localPath,
  }) {
    final $result = create();
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    if (remotePath != null) {
      $result.remotePath = remotePath;
    }
    if (localPath != null) {
      $result.localPath = localPath;
    }
    return $result;
  }
  FileTransferRequest._() : super();
  factory FileTransferRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileTransferRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileTransferRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..aOS(2, _omitFieldNames ? '' : 'remotePath')
    ..aOS(3, _omitFieldNames ? '' : 'localPath')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileTransferRequest clone() => FileTransferRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileTransferRequest copyWith(void Function(FileTransferRequest) updates) =>
      super.copyWith((message) => updates(message as FileTransferRequest))
          as FileTransferRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileTransferRequest create() => FileTransferRequest._();
  FileTransferRequest createEmptyInstance() => create();
  static $pb.PbList<FileTransferRequest> createRepeated() =>
      $pb.PbList<FileTransferRequest>();
  @$core.pragma('dart2js:noInline')
  static FileTransferRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileTransferRequest>(create);
  static FileTransferRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get remotePath => $_getSZ(1);
  @$pb.TagNumber(2)
  set remotePath($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasRemotePath() => $_has(1);
  @$pb.TagNumber(2)
  void clearRemotePath() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get localPath => $_getSZ(2);
  @$pb.TagNumber(3)
  set localPath($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasLocalPath() => $_has(2);
  @$pb.TagNumber(3)
  void clearLocalPath() => clearField(3);
}

class FileTransferStatus_Progress extends $pb.GeneratedMessage {
  factory FileTransferStatus_Progress({
    $core.int? bytesRead,
  }) {
    final $result = create();
    if (bytesRead != null) {
      $result.bytesRead = bytesRead;
    }
    return $result;
  }
  FileTransferStatus_Progress._() : super();
  factory FileTransferStatus_Progress.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileTransferStatus_Progress.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileTransferStatus.Progress',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'bytesRead', $pb.PbFieldType.O3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileTransferStatus_Progress clone() =>
      FileTransferStatus_Progress()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileTransferStatus_Progress copyWith(
          void Function(FileTransferStatus_Progress) updates) =>
      super.copyWith(
              (message) => updates(message as FileTransferStatus_Progress))
          as FileTransferStatus_Progress;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileTransferStatus_Progress create() =>
      FileTransferStatus_Progress._();
  FileTransferStatus_Progress createEmptyInstance() => create();
  static $pb.PbList<FileTransferStatus_Progress> createRepeated() =>
      $pb.PbList<FileTransferStatus_Progress>();
  @$core.pragma('dart2js:noInline')
  static FileTransferStatus_Progress getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileTransferStatus_Progress>(create);
  static FileTransferStatus_Progress? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get bytesRead => $_getIZ(0);
  @$pb.TagNumber(1)
  set bytesRead($core.int v) {
    $_setSignedInt32(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasBytesRead() => $_has(0);
  @$pb.TagNumber(1)
  void clearBytesRead() => clearField(1);
}

class FileTransferStatus_Completed extends $pb.GeneratedMessage {
  factory FileTransferStatus_Completed() => create();
  FileTransferStatus_Completed._() : super();
  factory FileTransferStatus_Completed.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileTransferStatus_Completed.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileTransferStatus.Completed',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileTransferStatus_Completed clone() =>
      FileTransferStatus_Completed()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileTransferStatus_Completed copyWith(
          void Function(FileTransferStatus_Completed) updates) =>
      super.copyWith(
              (message) => updates(message as FileTransferStatus_Completed))
          as FileTransferStatus_Completed;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileTransferStatus_Completed create() =>
      FileTransferStatus_Completed._();
  FileTransferStatus_Completed createEmptyInstance() => create();
  static $pb.PbList<FileTransferStatus_Completed> createRepeated() =>
      $pb.PbList<FileTransferStatus_Completed>();
  @$core.pragma('dart2js:noInline')
  static FileTransferStatus_Completed getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileTransferStatus_Completed>(create);
  static FileTransferStatus_Completed? _defaultInstance;
}

enum FileTransferStatus_Typ { progress, completed, notSet }

class FileTransferStatus extends $pb.GeneratedMessage {
  factory FileTransferStatus({
    FileTransferStatus_Progress? progress,
    FileTransferStatus_Completed? completed,
  }) {
    final $result = create();
    if (progress != null) {
      $result.progress = progress;
    }
    if (completed != null) {
      $result.completed = completed;
    }
    return $result;
  }
  FileTransferStatus._() : super();
  factory FileTransferStatus.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileTransferStatus.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, FileTransferStatus_Typ>
      _FileTransferStatus_TypByTag = {
    1: FileTransferStatus_Typ.progress,
    2: FileTransferStatus_Typ.completed,
    0: FileTransferStatus_Typ.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileTransferStatus',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<FileTransferStatus_Progress>(1, _omitFieldNames ? '' : 'progress',
        subBuilder: FileTransferStatus_Progress.create)
    ..aOM<FileTransferStatus_Completed>(2, _omitFieldNames ? '' : 'completed',
        subBuilder: FileTransferStatus_Completed.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileTransferStatus clone() => FileTransferStatus()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileTransferStatus copyWith(void Function(FileTransferStatus) updates) =>
      super.copyWith((message) => updates(message as FileTransferStatus))
          as FileTransferStatus;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileTransferStatus create() => FileTransferStatus._();
  FileTransferStatus createEmptyInstance() => create();
  static $pb.PbList<FileTransferStatus> createRepeated() =>
      $pb.PbList<FileTransferStatus>();
  @$core.pragma('dart2js:noInline')
  static FileTransferStatus getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileTransferStatus>(create);
  static FileTransferStatus? _defaultInstance;

  FileTransferStatus_Typ whichTyp() =>
      _FileTransferStatus_TypByTag[$_whichOneof(0)]!;
  void clearTyp() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  FileTransferStatus_Progress get progress => $_getN(0);
  @$pb.TagNumber(1)
  set progress(FileTransferStatus_Progress v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasProgress() => $_has(0);
  @$pb.TagNumber(1)
  void clearProgress() => clearField(1);
  @$pb.TagNumber(1)
  FileTransferStatus_Progress ensureProgress() => $_ensure(0);

  @$pb.TagNumber(2)
  FileTransferStatus_Completed get completed => $_getN(1);
  @$pb.TagNumber(2)
  set completed(FileTransferStatus_Completed v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasCompleted() => $_has(1);
  @$pb.TagNumber(2)
  void clearCompleted() => clearField(2);
  @$pb.TagNumber(2)
  FileTransferStatus_Completed ensureCompleted() => $_ensure(1);
}

class FileWriteRequest extends $pb.GeneratedMessage {
  factory FileWriteRequest({
    $core.String? fileHandleId,
    $core.List<$core.int>? data,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (fileHandleId != null) {
      $result.fileHandleId = fileHandleId;
    }
    if (data != null) {
      $result.data = data;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  FileWriteRequest._() : super();
  factory FileWriteRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileWriteRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileWriteRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'fileHandleId')
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileWriteRequest clone() => FileWriteRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileWriteRequest copyWith(void Function(FileWriteRequest) updates) =>
      super.copyWith((message) => updates(message as FileWriteRequest))
          as FileWriteRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileWriteRequest create() => FileWriteRequest._();
  FileWriteRequest createEmptyInstance() => create();
  static $pb.PbList<FileWriteRequest> createRepeated() =>
      $pb.PbList<FileWriteRequest>();
  @$core.pragma('dart2js:noInline')
  static FileWriteRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileWriteRequest>(create);
  static FileWriteRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get fileHandleId => $_getSZ(0);
  @$pb.TagNumber(1)
  set fileHandleId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasFileHandleId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileHandleId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get data => $_getN(1);
  @$pb.TagNumber(2)
  set data($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasData() => $_has(1);
  @$pb.TagNumber(2)
  void clearData() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get sessionId => $_getSZ(2);
  @$pb.TagNumber(3)
  set sessionId($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasSessionId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSessionId() => clearField(3);
}

class FileWriteResponse extends $pb.GeneratedMessage {
  factory FileWriteResponse({
    $core.bool? success,
  }) {
    final $result = create();
    if (success != null) {
      $result.success = success;
    }
    return $result;
  }
  FileWriteResponse._() : super();
  factory FileWriteResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileWriteResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileWriteResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileWriteResponse clone() => FileWriteResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileWriteResponse copyWith(void Function(FileWriteResponse) updates) =>
      super.copyWith((message) => updates(message as FileWriteResponse))
          as FileWriteResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileWriteResponse create() => FileWriteResponse._();
  FileWriteResponse createEmptyInstance() => create();
  static $pb.PbList<FileWriteResponse> createRepeated() =>
      $pb.PbList<FileWriteResponse>();
  @$core.pragma('dart2js:noInline')
  static FileWriteResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileWriteResponse>(create);
  static FileWriteResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) {
    $_setBool(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);
}

class FileReadRequest extends $pb.GeneratedMessage {
  factory FileReadRequest({
    $core.String? fileHandleId,
    $core.int? bufSize,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (fileHandleId != null) {
      $result.fileHandleId = fileHandleId;
    }
    if (bufSize != null) {
      $result.bufSize = bufSize;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  FileReadRequest._() : super();
  factory FileReadRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileReadRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileReadRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'fileHandleId')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'bufSize', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileReadRequest clone() => FileReadRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileReadRequest copyWith(void Function(FileReadRequest) updates) =>
      super.copyWith((message) => updates(message as FileReadRequest))
          as FileReadRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileReadRequest create() => FileReadRequest._();
  FileReadRequest createEmptyInstance() => create();
  static $pb.PbList<FileReadRequest> createRepeated() =>
      $pb.PbList<FileReadRequest>();
  @$core.pragma('dart2js:noInline')
  static FileReadRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileReadRequest>(create);
  static FileReadRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get fileHandleId => $_getSZ(0);
  @$pb.TagNumber(1)
  set fileHandleId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasFileHandleId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileHandleId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get bufSize => $_getIZ(1);
  @$pb.TagNumber(2)
  set bufSize($core.int v) {
    $_setSignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasBufSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearBufSize() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get sessionId => $_getSZ(2);
  @$pb.TagNumber(3)
  set sessionId($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasSessionId() => $_has(2);
  @$pb.TagNumber(3)
  void clearSessionId() => clearField(3);
}

class FileReadResponse extends $pb.GeneratedMessage {
  factory FileReadResponse({
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  FileReadResponse._() : super();
  factory FileReadResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileReadResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileReadResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileReadResponse clone() => FileReadResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileReadResponse copyWith(void Function(FileReadResponse) updates) =>
      super.copyWith((message) => updates(message as FileReadResponse))
          as FileReadResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileReadResponse create() => FileReadResponse._();
  FileReadResponse createEmptyInstance() => create();
  static $pb.PbList<FileReadResponse> createRepeated() =>
      $pb.PbList<FileReadResponse>();
  @$core.pragma('dart2js:noInline')
  static FileReadResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileReadResponse>(create);
  static FileReadResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class FileCloseResponse extends $pb.GeneratedMessage {
  factory FileCloseResponse({
    $core.bool? success,
  }) {
    final $result = create();
    if (success != null) {
      $result.success = success;
    }
    return $result;
  }
  FileCloseResponse._() : super();
  factory FileCloseResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileCloseResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileCloseResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOB(1, _omitFieldNames ? '' : 'success')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileCloseResponse clone() => FileCloseResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileCloseResponse copyWith(void Function(FileCloseResponse) updates) =>
      super.copyWith((message) => updates(message as FileCloseResponse))
          as FileCloseResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileCloseResponse create() => FileCloseResponse._();
  FileCloseResponse createEmptyInstance() => create();
  static $pb.PbList<FileCloseResponse> createRepeated() =>
      $pb.PbList<FileCloseResponse>();
  @$core.pragma('dart2js:noInline')
  static FileCloseResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileCloseResponse>(create);
  static FileCloseResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get success => $_getBF(0);
  @$pb.TagNumber(1)
  set success($core.bool v) {
    $_setBool(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSuccess() => $_has(0);
  @$pb.TagNumber(1)
  void clearSuccess() => clearField(1);
}

class FileMetadataRequest extends $pb.GeneratedMessage {
  factory FileMetadataRequest({
    $core.String? path,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (path != null) {
      $result.path = path;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  FileMetadataRequest._() : super();
  factory FileMetadataRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileMetadataRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileMetadataRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..aOS(2, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileMetadataRequest clone() => FileMetadataRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileMetadataRequest copyWith(void Function(FileMetadataRequest) updates) =>
      super.copyWith((message) => updates(message as FileMetadataRequest))
          as FileMetadataRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileMetadataRequest create() => FileMetadataRequest._();
  FileMetadataRequest createEmptyInstance() => create();
  static $pb.PbList<FileMetadataRequest> createRepeated() =>
      $pb.PbList<FileMetadataRequest>();
  @$core.pragma('dart2js:noInline')
  static FileMetadataRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileMetadataRequest>(create);
  static FileMetadataRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionId => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => clearField(2);
}

class FileMetadataResponse extends $pb.GeneratedMessage {
  factory FileMetadataResponse({
    $core.String? path,
    $fixnum.Int64? size,
    $fixnum.Int64? lastModified,
    $core.bool? isDirectory,
  }) {
    final $result = create();
    if (path != null) {
      $result.path = path;
    }
    if (size != null) {
      $result.size = size;
    }
    if (lastModified != null) {
      $result.lastModified = lastModified;
    }
    if (isDirectory != null) {
      $result.isDirectory = isDirectory;
    }
    return $result;
  }
  FileMetadataResponse._() : super();
  factory FileMetadataResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileMetadataResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileMetadataResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'size', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(
        3, _omitFieldNames ? '' : 'lastModified', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(4, _omitFieldNames ? '' : 'isDirectory')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileMetadataResponse clone() =>
      FileMetadataResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileMetadataResponse copyWith(void Function(FileMetadataResponse) updates) =>
      super.copyWith((message) => updates(message as FileMetadataResponse))
          as FileMetadataResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileMetadataResponse create() => FileMetadataResponse._();
  FileMetadataResponse createEmptyInstance() => create();
  static $pb.PbList<FileMetadataResponse> createRepeated() =>
      $pb.PbList<FileMetadataResponse>();
  @$core.pragma('dart2js:noInline')
  static FileMetadataResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<FileMetadataResponse>(create);
  static FileMetadataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get size => $_getI64(1);
  @$pb.TagNumber(2)
  set size($fixnum.Int64 v) {
    $_setInt64(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearSize() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get lastModified => $_getI64(2);
  @$pb.TagNumber(3)
  set lastModified($fixnum.Int64 v) {
    $_setInt64(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasLastModified() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastModified() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get isDirectory => $_getBF(3);
  @$pb.TagNumber(4)
  set isDirectory($core.bool v) {
    $_setBool(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasIsDirectory() => $_has(3);
  @$pb.TagNumber(4)
  void clearIsDirectory() => clearField(4);
}

class DirMetadata extends $pb.GeneratedMessage {
  factory DirMetadata({
    $core.String? path,
  }) {
    final $result = create();
    if (path != null) {
      $result.path = path;
    }
    return $result;
  }
  DirMetadata._() : super();
  factory DirMetadata.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DirMetadata.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'DirMetadata',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'path')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DirMetadata clone() => DirMetadata()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DirMetadata copyWith(void Function(DirMetadata) updates) =>
      super.copyWith((message) => updates(message as DirMetadata))
          as DirMetadata;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static DirMetadata create() => DirMetadata._();
  DirMetadata createEmptyInstance() => create();
  static $pb.PbList<DirMetadata> createRepeated() => $pb.PbList<DirMetadata>();
  @$core.pragma('dart2js:noInline')
  static DirMetadata getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DirMetadata>(create);
  static DirMetadata? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);
}

class FileList extends $pb.GeneratedMessage {
  factory FileList({
    $core.Iterable<FileData>? files,
  }) {
    final $result = create();
    if (files != null) {
      $result.files.addAll(files);
    }
    return $result;
  }
  FileList._() : super();
  factory FileList.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileList.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileList',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..pc<FileData>(1, _omitFieldNames ? '' : 'files', $pb.PbFieldType.PM,
        subBuilder: FileData.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileList clone() => FileList()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileList copyWith(void Function(FileList) updates) =>
      super.copyWith((message) => updates(message as FileList)) as FileList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileList create() => FileList._();
  FileList createEmptyInstance() => create();
  static $pb.PbList<FileList> createRepeated() => $pb.PbList<FileList>();
  @$core.pragma('dart2js:noInline')
  static FileList getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileList>(create);
  static FileList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FileData> get files => $_getList(0);
}

class FileData extends $pb.GeneratedMessage {
  factory FileData({
    $core.String? fileName,
    $fixnum.Int64? fileSize,
    $core.String? filePath,
    $core.bool? isDir,
  }) {
    final $result = create();
    if (fileName != null) {
      $result.fileName = fileName;
    }
    if (fileSize != null) {
      $result.fileSize = fileSize;
    }
    if (filePath != null) {
      $result.filePath = filePath;
    }
    if (isDir != null) {
      $result.isDir = isDir;
    }
    return $result;
  }
  FileData._() : super();
  factory FileData.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory FileData.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'FileData',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'fileName')
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'fileSize', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(3, _omitFieldNames ? '' : 'filePath')
    ..aOB(4, _omitFieldNames ? '' : 'isDir')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  FileData clone() => FileData()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  FileData copyWith(void Function(FileData) updates) =>
      super.copyWith((message) => updates(message as FileData)) as FileData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FileData create() => FileData._();
  FileData createEmptyInstance() => create();
  static $pb.PbList<FileData> createRepeated() => $pb.PbList<FileData>();
  @$core.pragma('dart2js:noInline')
  static FileData getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FileData>(create);
  static FileData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get fileName => $_getSZ(0);
  @$pb.TagNumber(1)
  set fileName($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasFileName() => $_has(0);
  @$pb.TagNumber(1)
  void clearFileName() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get fileSize => $_getI64(1);
  @$pb.TagNumber(2)
  set fileSize($fixnum.Int64 v) {
    $_setInt64(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasFileSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearFileSize() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get filePath => $_getSZ(2);
  @$pb.TagNumber(3)
  set filePath($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasFilePath() => $_has(2);
  @$pb.TagNumber(3)
  void clearFilePath() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get isDir => $_getBF(3);
  @$pb.TagNumber(4)
  set isDir($core.bool v) {
    $_setBool(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasIsDir() => $_has(3);
  @$pb.TagNumber(4)
  void clearIsDir() => clearField(4);
}

class PtyRequestResponse extends $pb.GeneratedMessage {
  factory PtyRequestResponse({
    $core.String? channelId,
  }) {
    final $result = create();
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  PtyRequestResponse._() : super();
  factory PtyRequestResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory PtyRequestResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PtyRequestResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'channelId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  PtyRequestResponse clone() => PtyRequestResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  PtyRequestResponse copyWith(void Function(PtyRequestResponse) updates) =>
      super.copyWith((message) => updates(message as PtyRequestResponse))
          as PtyRequestResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PtyRequestResponse create() => PtyRequestResponse._();
  PtyRequestResponse createEmptyInstance() => create();
  static $pb.PbList<PtyRequestResponse> createRepeated() =>
      $pb.PbList<PtyRequestResponse>();
  @$core.pragma('dart2js:noInline')
  static PtyRequestResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PtyRequestResponse>(create);
  static PtyRequestResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get channelId => $_getSZ(0);
  @$pb.TagNumber(1)
  set channelId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class GenKeysRequest extends $pb.GeneratedMessage {
  factory GenKeysRequest() => create();
  GenKeysRequest._() : super();
  factory GenKeysRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GenKeysRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GenKeysRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GenKeysRequest clone() => GenKeysRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GenKeysRequest copyWith(void Function(GenKeysRequest) updates) =>
      super.copyWith((message) => updates(message as GenKeysRequest))
          as GenKeysRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GenKeysRequest create() => GenKeysRequest._();
  GenKeysRequest createEmptyInstance() => create();
  static $pb.PbList<GenKeysRequest> createRepeated() =>
      $pb.PbList<GenKeysRequest>();
  @$core.pragma('dart2js:noInline')
  static GenKeysRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GenKeysRequest>(create);
  static GenKeysRequest? _defaultInstance;
}

class GenKeysResponse extends $pb.GeneratedMessage {
  factory GenKeysResponse() => create();
  GenKeysResponse._() : super();
  factory GenKeysResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GenKeysResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GenKeysResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GenKeysResponse clone() => GenKeysResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GenKeysResponse copyWith(void Function(GenKeysResponse) updates) =>
      super.copyWith((message) => updates(message as GenKeysResponse))
          as GenKeysResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GenKeysResponse create() => GenKeysResponse._();
  GenKeysResponse createEmptyInstance() => create();
  static $pb.PbList<GenKeysResponse> createRepeated() =>
      $pb.PbList<GenKeysResponse>();
  @$core.pragma('dart2js:noInline')
  static GenKeysResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GenKeysResponse>(create);
  static GenKeysResponse? _defaultInstance;
}

class StreamRequest extends $pb.GeneratedMessage {
  factory StreamRequest({
    $core.List<$core.int>? data,
    $core.String? sessionId,
  }) {
    final $result = create();
    if (data != null) {
      $result.data = data;
    }
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  StreamRequest._() : super();
  factory StreamRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory StreamRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StreamRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  StreamRequest clone() => StreamRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  StreamRequest copyWith(void Function(StreamRequest) updates) =>
      super.copyWith((message) => updates(message as StreamRequest))
          as StreamRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StreamRequest create() => StreamRequest._();
  StreamRequest createEmptyInstance() => create();
  static $pb.PbList<StreamRequest> createRepeated() =>
      $pb.PbList<StreamRequest>();
  @$core.pragma('dart2js:noInline')
  static StreamRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StreamRequest>(create);
  static StreamRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionId => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSessionId() => $_has(1);
  @$pb.TagNumber(2)
  void clearSessionId() => clearField(2);
}

class StreamResponse extends $pb.GeneratedMessage {
  factory StreamResponse({
    $core.List<$core.int>? data,
  }) {
    final $result = create();
    if (data != null) {
      $result.data = data;
    }
    return $result;
  }
  StreamResponse._() : super();
  factory StreamResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory StreamResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StreamResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  StreamResponse clone() => StreamResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  StreamResponse copyWith(void Function(StreamResponse) updates) =>
      super.copyWith((message) => updates(message as StreamResponse))
          as StreamResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StreamResponse create() => StreamResponse._();
  StreamResponse createEmptyInstance() => create();
  static $pb.PbList<StreamResponse> createRepeated() =>
      $pb.PbList<StreamResponse>();
  @$core.pragma('dart2js:noInline')
  static StreamResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StreamResponse>(create);
  static StreamResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class NewSessionRequest extends $pb.GeneratedMessage {
  factory NewSessionRequest({
    $core.String? privateKey,
    $core.String? knownHostsPath,
    SessionData? sessionData,
  }) {
    final $result = create();
    if (privateKey != null) {
      $result.privateKey = privateKey;
    }
    if (knownHostsPath != null) {
      $result.knownHostsPath = knownHostsPath;
    }
    if (sessionData != null) {
      $result.sessionData = sessionData;
    }
    return $result;
  }
  NewSessionRequest._() : super();
  factory NewSessionRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory NewSessionRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NewSessionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(3, _omitFieldNames ? '' : 'privateKey')
    ..aOS(4, _omitFieldNames ? '' : 'knownHostsPath')
    ..aOM<SessionData>(5, _omitFieldNames ? '' : 'sessionData',
        subBuilder: SessionData.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  NewSessionRequest clone() => NewSessionRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  NewSessionRequest copyWith(void Function(NewSessionRequest) updates) =>
      super.copyWith((message) => updates(message as NewSessionRequest))
          as NewSessionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSessionRequest create() => NewSessionRequest._();
  NewSessionRequest createEmptyInstance() => create();
  static $pb.PbList<NewSessionRequest> createRepeated() =>
      $pb.PbList<NewSessionRequest>();
  @$core.pragma('dart2js:noInline')
  static NewSessionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NewSessionRequest>(create);
  static NewSessionRequest? _defaultInstance;

  @$pb.TagNumber(3)
  $core.String get privateKey => $_getSZ(0);
  @$pb.TagNumber(3)
  set privateKey($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasPrivateKey() => $_has(0);
  @$pb.TagNumber(3)
  void clearPrivateKey() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get knownHostsPath => $_getSZ(1);
  @$pb.TagNumber(4)
  set knownHostsPath($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasKnownHostsPath() => $_has(1);
  @$pb.TagNumber(4)
  void clearKnownHostsPath() => clearField(4);

  @$pb.TagNumber(5)
  SessionData get sessionData => $_getN(2);
  @$pb.TagNumber(5)
  set sessionData(SessionData v) {
    setField(5, v);
  }

  @$pb.TagNumber(5)
  $core.bool hasSessionData() => $_has(2);
  @$pb.TagNumber(5)
  void clearSessionData() => clearField(5);
  @$pb.TagNumber(5)
  SessionData ensureSessionData() => $_ensure(2);
}

class NewSessionResponse extends $pb.GeneratedMessage {
  factory NewSessionResponse({
    $core.String? sessionId,
  }) {
    final $result = create();
    if (sessionId != null) {
      $result.sessionId = sessionId;
    }
    return $result;
  }
  NewSessionResponse._() : super();
  factory NewSessionResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory NewSessionResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NewSessionResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  NewSessionResponse clone() => NewSessionResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  NewSessionResponse copyWith(void Function(NewSessionResponse) updates) =>
      super.copyWith((message) => updates(message as NewSessionResponse))
          as NewSessionResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSessionResponse create() => NewSessionResponse._();
  NewSessionResponse createEmptyInstance() => create();
  static $pb.PbList<NewSessionResponse> createRepeated() =>
      $pb.PbList<NewSessionResponse>();
  @$core.pragma('dart2js:noInline')
  static NewSessionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NewSessionResponse>(create);
  static NewSessionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => clearField(1);
}

/// Opening a new connection locally
class NewConnectionRequest extends $pb.GeneratedMessage {
  factory NewConnectionRequest({
    $core.String? coordinatorUrl,
    $core.String? targetId,
    $core.String? ownIpv6,
  }) {
    final $result = create();
    if (coordinatorUrl != null) {
      $result.coordinatorUrl = coordinatorUrl;
    }
    if (targetId != null) {
      $result.targetId = targetId;
    }
    if (ownIpv6 != null) {
      $result.ownIpv6 = ownIpv6;
    }
    return $result;
  }
  NewConnectionRequest._() : super();
  factory NewConnectionRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory NewConnectionRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NewConnectionRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'coordinatorUrl')
    ..aOS(2, _omitFieldNames ? '' : 'targetId')
    ..aOS(3, _omitFieldNames ? '' : 'ownIpv6')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  NewConnectionRequest clone() =>
      NewConnectionRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  NewConnectionRequest copyWith(void Function(NewConnectionRequest) updates) =>
      super.copyWith((message) => updates(message as NewConnectionRequest))
          as NewConnectionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewConnectionRequest create() => NewConnectionRequest._();
  NewConnectionRequest createEmptyInstance() => create();
  static $pb.PbList<NewConnectionRequest> createRepeated() =>
      $pb.PbList<NewConnectionRequest>();
  @$core.pragma('dart2js:noInline')
  static NewConnectionRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NewConnectionRequest>(create);
  static NewConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get coordinatorUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set coordinatorUrl($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasCoordinatorUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearCoordinatorUrl() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetId($core.String v) {
    $_setString(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasTargetId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetId() => clearField(2);

  /// There's no good way of obtaining ipv6 in rust in both android and linux so we're passing it here
  @$pb.TagNumber(3)
  $core.String get ownIpv6 => $_getSZ(2);
  @$pb.TagNumber(3)
  set ownIpv6($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasOwnIpv6() => $_has(2);
  @$pb.TagNumber(3)
  void clearOwnIpv6() => clearField(3);
}

class NewConnectionResponse extends $pb.GeneratedMessage {
  factory NewConnectionResponse({
    $core.String? connectionId,
  }) {
    final $result = create();
    if (connectionId != null) {
      $result.connectionId = connectionId;
    }
    return $result;
  }
  NewConnectionResponse._() : super();
  factory NewConnectionResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory NewConnectionResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'NewConnectionResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'connectionId')
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  NewConnectionResponse clone() =>
      NewConnectionResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  NewConnectionResponse copyWith(
          void Function(NewConnectionResponse) updates) =>
      super.copyWith((message) => updates(message as NewConnectionResponse))
          as NewConnectionResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewConnectionResponse create() => NewConnectionResponse._();
  NewConnectionResponse createEmptyInstance() => create();
  static $pb.PbList<NewConnectionResponse> createRepeated() =>
      $pb.PbList<NewConnectionResponse>();
  @$core.pragma('dart2js:noInline')
  static NewConnectionResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<NewConnectionResponse>(create);
  static NewConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get connectionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set connectionId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasConnectionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnectionId() => clearField(1);
}

const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
