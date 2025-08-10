import { startRegistration, startAuthentication, browserSupportsWebAuthn } from '@simplewebauthn/browser';
import type { 
  PublicKeyCredentialCreationOptionsJSON,
  PublicKeyCredentialRequestOptionsJSON,
  RegistrationResponseJSON,
  AuthenticationResponseJSON 
} from '@simplewebauthn/browser';

// Re-export the browser support check
export { browserSupportsWebAuthn as isWebAuthnSupported };

// Create a new passkey using SimpleWebAuthn
export async function createPasskey(optionsJSON: PublicKeyCredentialCreationOptionsJSON): Promise<RegistrationResponseJSON> {
  try {
    const attResp = await startRegistration({ optionsJSON });
    return attResp;
  } catch (error: any) {
    console.error('Passkey creation error:', error);
    throw error;
  }
}

// Authenticate with a passkey using SimpleWebAuthn
export async function authenticatePasskey(optionsJSON: PublicKeyCredentialRequestOptionsJSON): Promise<AuthenticationResponseJSON> {
  try {
    const assResp = await startAuthentication({ optionsJSON });
    return assResp;
  } catch (error: any) {
    console.error('Passkey authentication error:', error);
    throw error;
  }
}

// Get user-friendly error messages
export function getWebAuthnErrorMessage(error: any): string {
  const errorString = error?.toString() || '';
  const errorName = error?.name || '';
  
  if (errorName === 'NotAllowedError' || errorString.includes('NotAllowedError')) {
    return 'The operation was cancelled or not allowed. Please try again.';
  } else if (errorName === 'InvalidStateError' || errorString.includes('InvalidStateError')) {
    return 'This authenticator may already be registered. Try a different one.';
  } else if (errorName === 'NotSupportedError' || errorString.includes('NotSupportedError')) {
    return 'This browser or device does not support passkeys.';
  } else if (errorName === 'SecurityError' || errorString.includes('SecurityError')) {
    return 'The operation is insecure. Please ensure you are using HTTPS.';
  } else if (errorName === 'AbortError' || errorString.includes('AbortError')) {
    return 'The operation was aborted. Please try again.';
  } else if (errorName === 'ConstraintError' || errorString.includes('ConstraintError')) {
    return 'The authenticator does not support the requested options.';
  } else if (errorName === 'UnknownError' || errorString.includes('UnknownError')) {
    return 'An unknown error occurred. Please try again.';
  } else if (errorName === 'TypeError' || errorString.includes('TypeError')) {
    return 'Invalid parameters provided. Please refresh and try again.';
  }
  
  return error?.message || 'An error occurred during the passkey operation.';
}