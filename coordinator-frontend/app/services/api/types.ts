// API Request/Response Types
export interface RegisterResponse {
  account_number: string;
}

export interface LoginRequest {
  account_number: string;
}

export interface LoginResponse {
  success: boolean;
  account_id?: string;
  message?: string;
}

export interface DeviceRequest {
  device_id: string;
  categories?: string[];
  // account_number now extracted from session cookie
}

export interface DeviceResponse {
  install_key: string;
}

export interface DevicesRequest {
  // account_number now extracted from session cookie - no fields needed
}

export interface Device {
  id: string;
  account_id: string;
  device_id: string;
  os_name?: string;
  public_key?: string;
  metadata: Record<string, any>;
  version?: string;
  created_at: string;
  updated_at: string;
  last_seen_at: string;
  signature?: string;
  signed_at?: string;
  signer_credential_id?: string;
  category_id?: string;
}

export interface DeviceWithCategory extends Device {
  category_name?: string;
}

// New interface for devices with multiple categories
export interface DeviceWithCategories extends Device {
  categories: Category[];
}

export interface Category {
  id: string;
  account_id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface DevicesResponse {
  devices: DeviceWithCategories[];
  categories: Category[];
}

export interface UpdateDeviceRequest {
  device_id: string;
  os_name?: string;
  categories?: string[];
}

export interface UpdateDeviceResponse {
  success: boolean;
  message?: string;
}

export interface DeleteDeviceRequest {
  device_id: string;
  // account_number now extracted from session cookie
}

export interface DeleteDeviceResponse {
  success: boolean;
  message: string;
}

export interface ApiError {
  message: string;
  status?: number;
}

// Import SimpleWebAuthn types
import type { 
  PublicKeyCredentialCreationOptionsJSON,
  PublicKeyCredentialRequestOptionsJSON,
  RegistrationResponseJSON,
  AuthenticationResponseJSON 
} from '@simplewebauthn/browser';

// WebAuthn Types
export interface WebAuthnRegisterStartRequest {
  account_number: string;
  username: string;
  display_name: string;
}

export interface WebAuthnRegisterStartResponse {
  session_id: string;
  creation_challenge: PublicKeyCredentialCreationOptionsJSON;
}

export interface WebAuthnRegisterFinishRequest {
  session_id: string;
  credential: RegistrationResponseJSON;
}

export interface WebAuthnRegisterFinishResponse {
  success: boolean;
  message?: string;
}

export interface WebAuthnAuthStartRequest {
  account_number?: string; // Optional for usernameless flow
}

export interface WebAuthnAuthStartResponse {
  session_id: string;
  request_challenge: PublicKeyCredentialRequestOptionsJSON;
}

export interface WebAuthnAuthFinishRequest {
  session_id: string;
  credential: AuthenticationResponseJSON;
}

export interface WebAuthnAuthFinishResponse {
  success: boolean;
  jwt_token?: string;
  message?: string;
}

// Single-step passkey registration types
export interface PasskeyRegisterRequest {
  username: string;
  display_name: string;
}

export interface PasskeyRegisterResponse {
  session_id: string;
  creation_challenge: PublicKeyCredentialCreationOptionsJSON;
}

export interface PasskeyRegisterFinishRequest {
  session_id: string;
  credential: RegistrationResponseJSON;
}

export interface PasskeyRegisterFinishResponse {
  success: boolean;
  message: string;
  jwt_token?: string;
}

// Device signing types
export interface DeviceSignInfo {
  device_id: string;
  public_key: string;
  os_name?: string;
  created_at: string;
}

export interface SignDeviceStartRequest {
  device_id: string;
}

export interface SignDeviceStartResponse {
  session_id: string;
  request_challenge: PublicKeyCredentialRequestOptionsJSON;
  device_info: DeviceSignInfo;
}

export interface SignDeviceFinishRequest {
  session_id: string;
  credential: AuthenticationResponseJSON;
}

export interface SignDeviceFinishResponse {
  success: boolean;
  message?: string;
}