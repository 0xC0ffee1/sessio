import { ApiClient, getApiClient } from './client';
import type {
  RegisterResponse,
  LoginRequest,
  LoginResponse,
  DeviceRequest,
  DeviceResponse,
  DeleteDeviceRequest,
  DeleteDeviceResponse,
  DevicesRequest,
  DevicesResponse,
  UpdateDeviceRequest,
  UpdateDeviceResponse,
  WebAuthnRegisterStartRequest,
  WebAuthnRegisterStartResponse,
  WebAuthnRegisterFinishRequest,
  WebAuthnRegisterFinishResponse,
  WebAuthnAuthStartRequest,
  WebAuthnAuthStartResponse,
  WebAuthnAuthFinishRequest,
  WebAuthnAuthFinishResponse,
  PasskeyRegisterRequest,
  PasskeyRegisterResponse,
  PasskeyRegisterFinishRequest,
  PasskeyRegisterFinishResponse,
  SignDeviceStartRequest,
  SignDeviceStartResponse,
  SignDeviceFinishRequest,
  SignDeviceFinishResponse,
} from './types';

class CoordinatorApi {
  private apiClient: ApiClient;

  constructor(apiClient: any) {
    this.apiClient = apiClient;
  }

  // Account registration
  register(): Promise<RegisterResponse> {
    return this.apiClient.post<RegisterResponse>('/register');
  }

  // Account login
  login(request: LoginRequest): Promise<LoginResponse> {
    return this.apiClient.post<LoginResponse>('/login', request);
  }

  // Create install key for device
  createDeviceInstallKey(request: DeviceRequest): Promise<DeviceResponse> {
    return this.apiClient.post<DeviceResponse>('/device', request);
  }

  // Get devices for account  
  getDevices(request: DevicesRequest): Promise<DevicesResponse> {
    return this.apiClient.post<DevicesResponse>('/devices', request);
  }

  // Delete device
  deleteDevice(request: DeleteDeviceRequest): Promise<DeleteDeviceResponse> {
    return this.apiClient.delete<DeleteDeviceResponse>('/device', request);
  }

  // Update device
  updateDevice(request: UpdateDeviceRequest): Promise<UpdateDeviceResponse> {
    return this.apiClient.patch<UpdateDeviceResponse>('/device', request);
  }

  // Health check
  health(): Promise<{ status: string; timestamp: string }> {
    return this.apiClient.get('/health');
  }

  // Single-step Passkey Registration (creates account + passkey in one flow)
  passkey = {
    // Start single-step passkey registration
    registerStart: (request: PasskeyRegisterRequest): Promise<PasskeyRegisterResponse> => {
      return this.apiClient.post<PasskeyRegisterResponse>('/passkey/register/start', request);
    },

    // Finish single-step passkey registration
    registerFinish: (request: PasskeyRegisterFinishRequest): Promise<PasskeyRegisterFinishResponse> => {
      return this.apiClient.post<PasskeyRegisterFinishResponse>('/passkey/register/finish', request);
    },

    authStart: (request: WebAuthnAuthStartRequest): Promise<WebAuthnAuthStartResponse> => {
      return this.apiClient.post<WebAuthnAuthStartResponse>('/webauthn/auth/start', request);
    },

    // Finish passkey authentication
    authFinish: (request: WebAuthnAuthFinishRequest): Promise<WebAuthnAuthFinishResponse> => {
      return this.apiClient.post<WebAuthnAuthFinishResponse>('/webauthn/auth/finish', request);
    },
  };

  // Device signing endpoints
  deviceSign = {
    // Start device signing with passkey
    start: (request: SignDeviceStartRequest): Promise<SignDeviceStartResponse> => {
      return this.apiClient.post<SignDeviceStartResponse>('/device/sign/start', request);
    },

    // Finish device signing
    finish: (request: SignDeviceFinishRequest): Promise<SignDeviceFinishResponse> => {
      return this.apiClient.post<SignDeviceFinishResponse>('/device/sign/finish', request);
    },
  };
}

// Factory function to create CoordinatorApi instance with JWT token from request
export async function getCoordinatorApi(request: Request): Promise<CoordinatorApi> {
  const apiClient = await getApiClient(request);
  return new CoordinatorApi(apiClient);
}