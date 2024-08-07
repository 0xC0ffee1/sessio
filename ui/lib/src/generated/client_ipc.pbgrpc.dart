//
//  Generated code. Do not modify.
//  source: client_ipc.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'client_ipc.pb.dart' as $0;

export 'client_ipc.pb.dart';

@$pb.GrpcServiceName('clientipc.ClientIPC')
class ClientIPCClient extends $grpc.Client {
  static final _$initClient = $grpc.ClientMethod<$0.InitData, $0.InitResponse>(
      '/clientipc.ClientIPC/InitClient',
      ($0.InitData value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.InitResponse.fromBuffer(value));
  static final _$newConnection =
      $grpc.ClientMethod<$0.NewConnectionRequest, $0.NewConnectionResponse>(
          '/clientipc.ClientIPC/NewConnection',
          ($0.NewConnectionRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.NewConnectionResponse.fromBuffer(value));
  static final _$newSession =
      $grpc.ClientMethod<$0.NewSessionRequest, $0.NewSessionResponse>(
          '/clientipc.ClientIPC/NewSession',
          ($0.NewSessionRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.NewSessionResponse.fromBuffer(value));
  static final _$genKeys =
      $grpc.ClientMethod<$0.GenKeysRequest, $0.GenKeysResponse>(
          '/clientipc.ClientIPC/GenKeys',
          ($0.GenKeysRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GenKeysResponse.fromBuffer(value));
  static final _$getPublicKey =
      $grpc.ClientMethod<$0.GetKeyRequest, $0.PublicKey>(
          '/clientipc.ClientIPC/GetPublicKey',
          ($0.GetKeyRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.PublicKey.fromBuffer(value));
  static final _$openChannel = $grpc.ClientMethod<$0.Msg, $0.Msg>(
      '/clientipc.ClientIPC/OpenChannel',
      ($0.Msg value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.Msg.fromBuffer(value));
  static final _$localPortForward =
      $grpc.ClientMethod<$0.SessionData, $0.LocalPortForwardResponse>(
          '/clientipc.ClientIPC/LocalPortForward',
          ($0.SessionData value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.LocalPortForwardResponse.fromBuffer(value));
  static final _$openSftpChannel =
      $grpc.ClientMethod<$0.SessionData, $0.SftpRequestResponse>(
          '/clientipc.ClientIPC/OpenSftpChannel',
          ($0.SessionData value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.SftpRequestResponse.fromBuffer(value));
  static final _$listDirectory = $grpc.ClientMethod<$0.Path, $0.FileList>(
      '/clientipc.ClientIPC/ListDirectory',
      ($0.Path value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.FileList.fromBuffer(value));
  static final _$fileDownload =
      $grpc.ClientMethod<$0.FileTransferRequest, $0.FileTransferStatus>(
          '/clientipc.ClientIPC/FileDownload',
          ($0.FileTransferRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FileTransferStatus.fromBuffer(value));
  static final _$fileUpload =
      $grpc.ClientMethod<$0.FileTransferRequest, $0.FileTransferStatus>(
          '/clientipc.ClientIPC/FileUpload',
          ($0.FileTransferRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FileTransferStatus.fromBuffer(value));
  static final _$getSettings =
      $grpc.ClientMethod<$0.SettingsRequest, $0.Settings>(
          '/clientipc.ClientIPC/GetSettings',
          ($0.SettingsRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.Settings.fromBuffer(value));
  static final _$getSaveData =
      $grpc.ClientMethod<$0.GetSaveDataRequest, $0.UserData>(
          '/clientipc.ClientIPC/GetSaveData',
          ($0.GetSaveDataRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.UserData.fromBuffer(value));
  static final _$saveSettings = $grpc.ClientMethod<$0.Settings, $0.Settings>(
      '/clientipc.ClientIPC/SaveSettings',
      ($0.Settings value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.Settings.fromBuffer(value));
  static final _$saveUserData = $grpc.ClientMethod<$0.UserData, $0.UserData>(
      '/clientipc.ClientIPC/SaveUserData',
      ($0.UserData value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.UserData.fromBuffer(value));
  static final _$getActiveSessions =
      $grpc.ClientMethod<$0.SessionRequest, $0.SessionMap>(
          '/clientipc.ClientIPC/GetActiveSessions',
          ($0.SessionRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.SessionMap.fromBuffer(value));

  ClientIPCClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$0.InitResponse> initClient($0.InitData request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$initClient, request, options: options);
  }

  $grpc.ResponseFuture<$0.NewConnectionResponse> newConnection(
      $0.NewConnectionRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$newConnection, request, options: options);
  }

  $grpc.ResponseFuture<$0.NewSessionResponse> newSession(
      $0.NewSessionRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$newSession, request, options: options);
  }

  $grpc.ResponseFuture<$0.GenKeysResponse> genKeys($0.GenKeysRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$genKeys, request, options: options);
  }

  $grpc.ResponseFuture<$0.PublicKey> getPublicKey($0.GetKeyRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getPublicKey, request, options: options);
  }

  $grpc.ResponseStream<$0.Msg> openChannel($async.Stream<$0.Msg> request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(_$openChannel, request, options: options);
  }

  $grpc.ResponseFuture<$0.LocalPortForwardResponse> localPortForward(
      $0.SessionData request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$localPortForward, request, options: options);
  }

  $grpc.ResponseFuture<$0.SftpRequestResponse> openSftpChannel(
      $0.SessionData request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$openSftpChannel, request, options: options);
  }

  $grpc.ResponseFuture<$0.FileList> listDirectory($0.Path request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listDirectory, request, options: options);
  }

  $grpc.ResponseStream<$0.FileTransferStatus> fileDownload(
      $0.FileTransferRequest request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$fileDownload, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseStream<$0.FileTransferStatus> fileUpload(
      $0.FileTransferRequest request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$fileUpload, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.Settings> getSettings($0.SettingsRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.UserData> getSaveData($0.GetSaveDataRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getSaveData, request, options: options);
  }

  $grpc.ResponseFuture<$0.Settings> saveSettings($0.Settings request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$saveSettings, request, options: options);
  }

  $grpc.ResponseFuture<$0.UserData> saveUserData($0.UserData request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$saveUserData, request, options: options);
  }

  $grpc.ResponseFuture<$0.SessionMap> getActiveSessions(
      $0.SessionRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getActiveSessions, request, options: options);
  }
}

@$pb.GrpcServiceName('clientipc.ClientIPC')
abstract class ClientIPCServiceBase extends $grpc.Service {
  $core.String get $name => 'clientipc.ClientIPC';

  ClientIPCServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.InitData, $0.InitResponse>(
        'InitClient',
        initClient_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.InitData.fromBuffer(value),
        ($0.InitResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.NewConnectionRequest, $0.NewConnectionResponse>(
            'NewConnection',
            newConnection_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.NewConnectionRequest.fromBuffer(value),
            ($0.NewConnectionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.NewSessionRequest, $0.NewSessionResponse>(
        'NewSession',
        newSession_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.NewSessionRequest.fromBuffer(value),
        ($0.NewSessionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GenKeysRequest, $0.GenKeysResponse>(
        'GenKeys',
        genKeys_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GenKeysRequest.fromBuffer(value),
        ($0.GenKeysResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetKeyRequest, $0.PublicKey>(
        'GetPublicKey',
        getPublicKey_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetKeyRequest.fromBuffer(value),
        ($0.PublicKey value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Msg, $0.Msg>(
        'OpenChannel',
        openChannel,
        true,
        true,
        ($core.List<$core.int> value) => $0.Msg.fromBuffer(value),
        ($0.Msg value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SessionData, $0.LocalPortForwardResponse>(
        'LocalPortForward',
        localPortForward_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SessionData.fromBuffer(value),
        ($0.LocalPortForwardResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SessionData, $0.SftpRequestResponse>(
        'OpenSftpChannel',
        openSftpChannel_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SessionData.fromBuffer(value),
        ($0.SftpRequestResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Path, $0.FileList>(
        'ListDirectory',
        listDirectory_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Path.fromBuffer(value),
        ($0.FileList value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.FileTransferRequest, $0.FileTransferStatus>(
            'FileDownload',
            fileDownload_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.FileTransferRequest.fromBuffer(value),
            ($0.FileTransferStatus value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.FileTransferRequest, $0.FileTransferStatus>(
            'FileUpload',
            fileUpload_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.FileTransferRequest.fromBuffer(value),
            ($0.FileTransferStatus value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SettingsRequest, $0.Settings>(
        'GetSettings',
        getSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SettingsRequest.fromBuffer(value),
        ($0.Settings value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetSaveDataRequest, $0.UserData>(
        'GetSaveData',
        getSaveData_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetSaveDataRequest.fromBuffer(value),
        ($0.UserData value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Settings, $0.Settings>(
        'SaveSettings',
        saveSettings_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Settings.fromBuffer(value),
        ($0.Settings value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.UserData, $0.UserData>(
        'SaveUserData',
        saveUserData_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.UserData.fromBuffer(value),
        ($0.UserData value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SessionRequest, $0.SessionMap>(
        'GetActiveSessions',
        getActiveSessions_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.SessionRequest.fromBuffer(value),
        ($0.SessionMap value) => value.writeToBuffer()));
  }

  $async.Future<$0.InitResponse> initClient_Pre(
      $grpc.ServiceCall call, $async.Future<$0.InitData> request) async {
    return initClient(call, await request);
  }

  $async.Future<$0.NewConnectionResponse> newConnection_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.NewConnectionRequest> request) async {
    return newConnection(call, await request);
  }

  $async.Future<$0.NewSessionResponse> newSession_Pre($grpc.ServiceCall call,
      $async.Future<$0.NewSessionRequest> request) async {
    return newSession(call, await request);
  }

  $async.Future<$0.GenKeysResponse> genKeys_Pre(
      $grpc.ServiceCall call, $async.Future<$0.GenKeysRequest> request) async {
    return genKeys(call, await request);
  }

  $async.Future<$0.PublicKey> getPublicKey_Pre(
      $grpc.ServiceCall call, $async.Future<$0.GetKeyRequest> request) async {
    return getPublicKey(call, await request);
  }

  $async.Future<$0.LocalPortForwardResponse> localPortForward_Pre(
      $grpc.ServiceCall call, $async.Future<$0.SessionData> request) async {
    return localPortForward(call, await request);
  }

  $async.Future<$0.SftpRequestResponse> openSftpChannel_Pre(
      $grpc.ServiceCall call, $async.Future<$0.SessionData> request) async {
    return openSftpChannel(call, await request);
  }

  $async.Future<$0.FileList> listDirectory_Pre(
      $grpc.ServiceCall call, $async.Future<$0.Path> request) async {
    return listDirectory(call, await request);
  }

  $async.Stream<$0.FileTransferStatus> fileDownload_Pre($grpc.ServiceCall call,
      $async.Future<$0.FileTransferRequest> request) async* {
    yield* fileDownload(call, await request);
  }

  $async.Stream<$0.FileTransferStatus> fileUpload_Pre($grpc.ServiceCall call,
      $async.Future<$0.FileTransferRequest> request) async* {
    yield* fileUpload(call, await request);
  }

  $async.Future<$0.Settings> getSettings_Pre(
      $grpc.ServiceCall call, $async.Future<$0.SettingsRequest> request) async {
    return getSettings(call, await request);
  }

  $async.Future<$0.UserData> getSaveData_Pre($grpc.ServiceCall call,
      $async.Future<$0.GetSaveDataRequest> request) async {
    return getSaveData(call, await request);
  }

  $async.Future<$0.Settings> saveSettings_Pre(
      $grpc.ServiceCall call, $async.Future<$0.Settings> request) async {
    return saveSettings(call, await request);
  }

  $async.Future<$0.UserData> saveUserData_Pre(
      $grpc.ServiceCall call, $async.Future<$0.UserData> request) async {
    return saveUserData(call, await request);
  }

  $async.Future<$0.SessionMap> getActiveSessions_Pre(
      $grpc.ServiceCall call, $async.Future<$0.SessionRequest> request) async {
    return getActiveSessions(call, await request);
  }

  $async.Future<$0.InitResponse> initClient(
      $grpc.ServiceCall call, $0.InitData request);
  $async.Future<$0.NewConnectionResponse> newConnection(
      $grpc.ServiceCall call, $0.NewConnectionRequest request);
  $async.Future<$0.NewSessionResponse> newSession(
      $grpc.ServiceCall call, $0.NewSessionRequest request);
  $async.Future<$0.GenKeysResponse> genKeys(
      $grpc.ServiceCall call, $0.GenKeysRequest request);
  $async.Future<$0.PublicKey> getPublicKey(
      $grpc.ServiceCall call, $0.GetKeyRequest request);
  $async.Stream<$0.Msg> openChannel(
      $grpc.ServiceCall call, $async.Stream<$0.Msg> request);
  $async.Future<$0.LocalPortForwardResponse> localPortForward(
      $grpc.ServiceCall call, $0.SessionData request);
  $async.Future<$0.SftpRequestResponse> openSftpChannel(
      $grpc.ServiceCall call, $0.SessionData request);
  $async.Future<$0.FileList> listDirectory(
      $grpc.ServiceCall call, $0.Path request);
  $async.Stream<$0.FileTransferStatus> fileDownload(
      $grpc.ServiceCall call, $0.FileTransferRequest request);
  $async.Stream<$0.FileTransferStatus> fileUpload(
      $grpc.ServiceCall call, $0.FileTransferRequest request);
  $async.Future<$0.Settings> getSettings(
      $grpc.ServiceCall call, $0.SettingsRequest request);
  $async.Future<$0.UserData> getSaveData(
      $grpc.ServiceCall call, $0.GetSaveDataRequest request);
  $async.Future<$0.Settings> saveSettings(
      $grpc.ServiceCall call, $0.Settings request);
  $async.Future<$0.UserData> saveUserData(
      $grpc.ServiceCall call, $0.UserData request);
  $async.Future<$0.SessionMap> getActiveSessions(
      $grpc.ServiceCall call, $0.SessionRequest request);
}
