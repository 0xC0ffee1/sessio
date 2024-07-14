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

import 'package:protobuf/protobuf.dart' as $pb;

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
  factory Msg_Data.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Msg_Data.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Msg.Data', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Msg_Data clone() => Msg_Data()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Msg_Data copyWith(void Function(Msg_Data) updates) => super.copyWith((message) => updates(message as Msg_Data)) as Msg_Data;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_Data create() => Msg_Data._();
  Msg_Data createEmptyInstance() => create();
  static $pb.PbList<Msg_Data> createRepeated() => $pb.PbList<Msg_Data>();
  @$core.pragma('dart2js:noInline')
  static Msg_Data getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg_Data>(create);
  static Msg_Data? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get payload => $_getN(0);
  @$pb.TagNumber(1)
  set payload($core.List<$core.int> v) { $_setBytes(0, v); }
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
  factory Msg_PtyRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Msg_PtyRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Msg.PtyRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'colWidth', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'rowHeight', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Msg_PtyRequest clone() => Msg_PtyRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Msg_PtyRequest copyWith(void Function(Msg_PtyRequest) updates) => super.copyWith((message) => updates(message as Msg_PtyRequest)) as Msg_PtyRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_PtyRequest create() => Msg_PtyRequest._();
  Msg_PtyRequest createEmptyInstance() => create();
  static $pb.PbList<Msg_PtyRequest> createRepeated() => $pb.PbList<Msg_PtyRequest>();
  @$core.pragma('dart2js:noInline')
  static Msg_PtyRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg_PtyRequest>(create);
  static Msg_PtyRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get colWidth => $_getIZ(0);
  @$pb.TagNumber(1)
  set colWidth($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasColWidth() => $_has(0);
  @$pb.TagNumber(1)
  void clearColWidth() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get rowHeight => $_getIZ(1);
  @$pb.TagNumber(2)
  set rowHeight($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasRowHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearRowHeight() => clearField(2);
}

class Msg_ShellRequest extends $pb.GeneratedMessage {
  factory Msg_ShellRequest() => create();
  Msg_ShellRequest._() : super();
  factory Msg_ShellRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Msg_ShellRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Msg.ShellRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Msg_ShellRequest clone() => Msg_ShellRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Msg_ShellRequest copyWith(void Function(Msg_ShellRequest) updates) => super.copyWith((message) => updates(message as Msg_ShellRequest)) as Msg_ShellRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_ShellRequest create() => Msg_ShellRequest._();
  Msg_ShellRequest createEmptyInstance() => create();
  static $pb.PbList<Msg_ShellRequest> createRepeated() => $pb.PbList<Msg_ShellRequest>();
  @$core.pragma('dart2js:noInline')
  static Msg_ShellRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg_ShellRequest>(create);
  static Msg_ShellRequest? _defaultInstance;
}

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
  factory Msg_ChannelInit.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Msg_ChannelInit.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Msg.ChannelInit', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Msg_ChannelInit clone() => Msg_ChannelInit()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Msg_ChannelInit copyWith(void Function(Msg_ChannelInit) updates) => super.copyWith((message) => updates(message as Msg_ChannelInit)) as Msg_ChannelInit;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg_ChannelInit create() => Msg_ChannelInit._();
  Msg_ChannelInit createEmptyInstance() => create();
  static $pb.PbList<Msg_ChannelInit> createRepeated() => $pb.PbList<Msg_ChannelInit>();
  @$core.pragma('dart2js:noInline')
  static Msg_ChannelInit getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg_ChannelInit>(create);
  static Msg_ChannelInit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSessionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearSessionId() => clearField(1);
}

enum Msg_Type {
  data, 
  ptyRequest, 
  shellRequest, 
  channelInit, 
  notSet
}

class Msg extends $pb.GeneratedMessage {
  factory Msg({
    Msg_Data? data,
    Msg_PtyRequest? ptyRequest,
    Msg_ShellRequest? shellRequest,
    Msg_ChannelInit? channelInit,
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
    return $result;
  }
  Msg._() : super();
  factory Msg.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Msg.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Msg_Type> _Msg_TypeByTag = {
    1 : Msg_Type.data,
    2 : Msg_Type.ptyRequest,
    3 : Msg_Type.shellRequest,
    4 : Msg_Type.channelInit,
    0 : Msg_Type.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Msg', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4])
    ..aOM<Msg_Data>(1, _omitFieldNames ? '' : 'data', subBuilder: Msg_Data.create)
    ..aOM<Msg_PtyRequest>(2, _omitFieldNames ? '' : 'ptyRequest', subBuilder: Msg_PtyRequest.create)
    ..aOM<Msg_ShellRequest>(3, _omitFieldNames ? '' : 'shellRequest', subBuilder: Msg_ShellRequest.create)
    ..aOM<Msg_ChannelInit>(4, _omitFieldNames ? '' : 'channelInit', subBuilder: Msg_ChannelInit.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Msg clone() => Msg()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Msg copyWith(void Function(Msg) updates) => super.copyWith((message) => updates(message as Msg)) as Msg;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Msg create() => Msg._();
  Msg createEmptyInstance() => create();
  static $pb.PbList<Msg> createRepeated() => $pb.PbList<Msg>();
  @$core.pragma('dart2js:noInline')
  static Msg getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Msg>(create);
  static Msg? _defaultInstance;

  Msg_Type whichType() => _Msg_TypeByTag[$_whichOneof(0)]!;
  void clearType() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Msg_Data get data => $_getN(0);
  @$pb.TagNumber(1)
  set data(Msg_Data v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
  @$pb.TagNumber(1)
  Msg_Data ensureData() => $_ensure(0);

  @$pb.TagNumber(2)
  Msg_PtyRequest get ptyRequest => $_getN(1);
  @$pb.TagNumber(2)
  set ptyRequest(Msg_PtyRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasPtyRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearPtyRequest() => clearField(2);
  @$pb.TagNumber(2)
  Msg_PtyRequest ensurePtyRequest() => $_ensure(1);

  @$pb.TagNumber(3)
  Msg_ShellRequest get shellRequest => $_getN(2);
  @$pb.TagNumber(3)
  set shellRequest(Msg_ShellRequest v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasShellRequest() => $_has(2);
  @$pb.TagNumber(3)
  void clearShellRequest() => clearField(3);
  @$pb.TagNumber(3)
  Msg_ShellRequest ensureShellRequest() => $_ensure(2);

  @$pb.TagNumber(4)
  Msg_ChannelInit get channelInit => $_getN(3);
  @$pb.TagNumber(4)
  set channelInit(Msg_ChannelInit v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasChannelInit() => $_has(3);
  @$pb.TagNumber(4)
  void clearChannelInit() => clearField(4);
  @$pb.TagNumber(4)
  Msg_ChannelInit ensureChannelInit() => $_ensure(3);
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
  factory PtyRequestResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PtyRequestResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'PtyRequestResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'channelId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PtyRequestResponse clone() => PtyRequestResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PtyRequestResponse copyWith(void Function(PtyRequestResponse) updates) => super.copyWith((message) => updates(message as PtyRequestResponse)) as PtyRequestResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PtyRequestResponse create() => PtyRequestResponse._();
  PtyRequestResponse createEmptyInstance() => create();
  static $pb.PbList<PtyRequestResponse> createRepeated() => $pb.PbList<PtyRequestResponse>();
  @$core.pragma('dart2js:noInline')
  static PtyRequestResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PtyRequestResponse>(create);
  static PtyRequestResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get channelId => $_getSZ(0);
  @$pb.TagNumber(1)
  set channelId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class GenKeysRequest extends $pb.GeneratedMessage {
  factory GenKeysRequest({
    $core.String? keyPath,
  }) {
    final $result = create();
    if (keyPath != null) {
      $result.keyPath = keyPath;
    }
    return $result;
  }
  GenKeysRequest._() : super();
  factory GenKeysRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GenKeysRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GenKeysRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'keyPath')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GenKeysRequest clone() => GenKeysRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GenKeysRequest copyWith(void Function(GenKeysRequest) updates) => super.copyWith((message) => updates(message as GenKeysRequest)) as GenKeysRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GenKeysRequest create() => GenKeysRequest._();
  GenKeysRequest createEmptyInstance() => create();
  static $pb.PbList<GenKeysRequest> createRepeated() => $pb.PbList<GenKeysRequest>();
  @$core.pragma('dart2js:noInline')
  static GenKeysRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GenKeysRequest>(create);
  static GenKeysRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get keyPath => $_getSZ(0);
  @$pb.TagNumber(1)
  set keyPath($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasKeyPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearKeyPath() => clearField(1);
}

class GenKeysResponse extends $pb.GeneratedMessage {
  factory GenKeysResponse({
    $core.String? keyPath,
  }) {
    final $result = create();
    if (keyPath != null) {
      $result.keyPath = keyPath;
    }
    return $result;
  }
  GenKeysResponse._() : super();
  factory GenKeysResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GenKeysResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GenKeysResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'keyPath')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GenKeysResponse clone() => GenKeysResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GenKeysResponse copyWith(void Function(GenKeysResponse) updates) => super.copyWith((message) => updates(message as GenKeysResponse)) as GenKeysResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GenKeysResponse create() => GenKeysResponse._();
  GenKeysResponse createEmptyInstance() => create();
  static $pb.PbList<GenKeysResponse> createRepeated() => $pb.PbList<GenKeysResponse>();
  @$core.pragma('dart2js:noInline')
  static GenKeysResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GenKeysResponse>(create);
  static GenKeysResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get keyPath => $_getSZ(0);
  @$pb.TagNumber(1)
  set keyPath($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasKeyPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearKeyPath() => clearField(1);
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
  factory StreamRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StreamRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'StreamRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StreamRequest clone() => StreamRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StreamRequest copyWith(void Function(StreamRequest) updates) => super.copyWith((message) => updates(message as StreamRequest)) as StreamRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StreamRequest create() => StreamRequest._();
  StreamRequest createEmptyInstance() => create();
  static $pb.PbList<StreamRequest> createRepeated() => $pb.PbList<StreamRequest>();
  @$core.pragma('dart2js:noInline')
  static StreamRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StreamRequest>(create);
  static StreamRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get sessionId => $_getSZ(1);
  @$pb.TagNumber(2)
  set sessionId($core.String v) { $_setString(1, v); }
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
  factory StreamResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory StreamResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'StreamResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  StreamResponse clone() => StreamResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  StreamResponse copyWith(void Function(StreamResponse) updates) => super.copyWith((message) => updates(message as StreamResponse)) as StreamResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StreamResponse create() => StreamResponse._();
  StreamResponse createEmptyInstance() => create();
  static $pb.PbList<StreamResponse> createRepeated() => $pb.PbList<StreamResponse>();
  @$core.pragma('dart2js:noInline')
  static StreamResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<StreamResponse>(create);
  static StreamResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class NewSessionRequest extends $pb.GeneratedMessage {
  factory NewSessionRequest({
    $core.String? connectionId,
    $core.String? username,
    $core.String? privateKey,
    $core.String? knownHostsPath,
  }) {
    final $result = create();
    if (connectionId != null) {
      $result.connectionId = connectionId;
    }
    if (username != null) {
      $result.username = username;
    }
    if (privateKey != null) {
      $result.privateKey = privateKey;
    }
    if (knownHostsPath != null) {
      $result.knownHostsPath = knownHostsPath;
    }
    return $result;
  }
  NewSessionRequest._() : super();
  factory NewSessionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSessionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewSessionRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'connectionId')
    ..aOS(2, _omitFieldNames ? '' : 'username')
    ..aOS(3, _omitFieldNames ? '' : 'privateKey')
    ..aOS(4, _omitFieldNames ? '' : 'knownHostsPath')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSessionRequest clone() => NewSessionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSessionRequest copyWith(void Function(NewSessionRequest) updates) => super.copyWith((message) => updates(message as NewSessionRequest)) as NewSessionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSessionRequest create() => NewSessionRequest._();
  NewSessionRequest createEmptyInstance() => create();
  static $pb.PbList<NewSessionRequest> createRepeated() => $pb.PbList<NewSessionRequest>();
  @$core.pragma('dart2js:noInline')
  static NewSessionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewSessionRequest>(create);
  static NewSessionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get connectionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set connectionId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConnectionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnectionId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get username => $_getSZ(1);
  @$pb.TagNumber(2)
  set username($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUsername() => $_has(1);
  @$pb.TagNumber(2)
  void clearUsername() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get privateKey => $_getSZ(2);
  @$pb.TagNumber(3)
  set privateKey($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPrivateKey() => $_has(2);
  @$pb.TagNumber(3)
  void clearPrivateKey() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get knownHostsPath => $_getSZ(3);
  @$pb.TagNumber(4)
  set knownHostsPath($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasKnownHostsPath() => $_has(3);
  @$pb.TagNumber(4)
  void clearKnownHostsPath() => clearField(4);
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
  factory NewSessionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewSessionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewSessionResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'sessionId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewSessionResponse clone() => NewSessionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewSessionResponse copyWith(void Function(NewSessionResponse) updates) => super.copyWith((message) => updates(message as NewSessionResponse)) as NewSessionResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewSessionResponse create() => NewSessionResponse._();
  NewSessionResponse createEmptyInstance() => create();
  static $pb.PbList<NewSessionResponse> createRepeated() => $pb.PbList<NewSessionResponse>();
  @$core.pragma('dart2js:noInline')
  static NewSessionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewSessionResponse>(create);
  static NewSessionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get sessionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set sessionId($core.String v) { $_setString(0, v); }
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
  }) {
    final $result = create();
    if (coordinatorUrl != null) {
      $result.coordinatorUrl = coordinatorUrl;
    }
    if (targetId != null) {
      $result.targetId = targetId;
    }
    return $result;
  }
  NewConnectionRequest._() : super();
  factory NewConnectionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewConnectionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewConnectionRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'coordinatorUrl')
    ..aOS(2, _omitFieldNames ? '' : 'targetId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewConnectionRequest clone() => NewConnectionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewConnectionRequest copyWith(void Function(NewConnectionRequest) updates) => super.copyWith((message) => updates(message as NewConnectionRequest)) as NewConnectionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewConnectionRequest create() => NewConnectionRequest._();
  NewConnectionRequest createEmptyInstance() => create();
  static $pb.PbList<NewConnectionRequest> createRepeated() => $pb.PbList<NewConnectionRequest>();
  @$core.pragma('dart2js:noInline')
  static NewConnectionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewConnectionRequest>(create);
  static NewConnectionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get coordinatorUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set coordinatorUrl($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCoordinatorUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearCoordinatorUrl() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get targetId => $_getSZ(1);
  @$pb.TagNumber(2)
  set targetId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetId() => clearField(2);
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
  factory NewConnectionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewConnectionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewConnectionResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'clientipc'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'connectionId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewConnectionResponse clone() => NewConnectionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewConnectionResponse copyWith(void Function(NewConnectionResponse) updates) => super.copyWith((message) => updates(message as NewConnectionResponse)) as NewConnectionResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewConnectionResponse create() => NewConnectionResponse._();
  NewConnectionResponse createEmptyInstance() => create();
  static $pb.PbList<NewConnectionResponse> createRepeated() => $pb.PbList<NewConnectionResponse>();
  @$core.pragma('dart2js:noInline')
  static NewConnectionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewConnectionResponse>(create);
  static NewConnectionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get connectionId => $_getSZ(0);
  @$pb.TagNumber(1)
  set connectionId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasConnectionId() => $_has(0);
  @$pb.TagNumber(1)
  void clearConnectionId() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
