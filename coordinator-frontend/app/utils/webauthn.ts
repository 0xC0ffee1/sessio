/**
 * WebAuthn utilities for browser passkey authentication
 */

// Check if WebAuthn is supported in this browser
export function isWebAuthnSupported(): boolean {
  return typeof window !== 'undefined' && 
         'credentials' in navigator && 
         'create' in navigator.credentials && 
         'get' in navigator.credentials;
}

// Convert base64url to ArrayBuffer
function base64urlToBuffer(base64url: string): ArrayBuffer {
  const padding = '='.repeat((4 - base64url.length % 4) % 4);
  const base64 = (base64url + padding)
    .replace(/-/g, '+')
    .replace(/_/g, '/');
  
  const rawData = window.atob(base64);
  const outputArray = new Uint8Array(rawData.length);
  
  for (let i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray.buffer;
}

// Convert ArrayBuffer to base64url
function bufferToBase64url(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let str = '';
  for (let i = 0; i < bytes.length; i++) {
    str += String.fromCharCode(bytes[i]);
  }
  
  return window.btoa(str)
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

// Create a new passkey (registration) - SimpleWebAuthn compatible
export async function createPasskey(options: any): Promise<any> {
  if (!isWebAuthnSupported()) {
    throw new Error('WebAuthn is not supported in this browser');
  }

  // Convert challenge and user.id from base64url to ArrayBuffer
  const publicKey = {
    ...options,
    challenge: base64urlToBuffer(options.challenge),
    user: {
      ...options.user,
      id: base64urlToBuffer(options.user.id),
    },
  };

  try {
    const credential = await navigator.credentials.create({ publicKey }) as PublicKeyCredential;
    
    if (!credential) {
      throw new Error('Failed to create credential');
    }

    const response = credential.response as AuthenticatorAttestationResponse;
    
    // Return in SimpleWebAuthn format
    return {
      id: credential.id,
      rawId: bufferToBase64url(credential.rawId),
      type: credential.type,
      response: {
        attestationObject: bufferToBase64url(response.attestationObject),
        clientDataJSON: bufferToBase64url(response.clientDataJSON),
        // Add transports if available
        transports: (credential.response as any).getTransports ? 
          (credential.response as any).getTransports() : undefined,
      },
      clientExtensionResults: credential.getClientExtensionResults(),
    };
  } catch (error: any) {
    console.error('Error creating passkey:', error);
    throw error;
  }
}

// Authenticate with existing passkey
export async function authenticatePasskey(options: any): Promise<any> {
  if (!isWebAuthnSupported()) {
    throw new Error('WebAuthn is not supported in this browser');
  }

  // Convert challenge and allowCredentials from base64url to ArrayBuffer
  const publicKey = {
    ...options,
    challenge: base64urlToBuffer(options.challenge),
    allowCredentials: options.allowCredentials?.map((cred: any) => ({
      ...cred,
      id: base64urlToBuffer(cred.id),
    })) || [],
  };

  try {
    const credential = await navigator.credentials.get({ publicKey }) as PublicKeyCredential;
    
    if (!credential) {
      throw new Error('Failed to get credential');
    }

    const response = credential.response as AuthenticatorAssertionResponse;
    
    return {
      id: credential.id,
      rawId: bufferToBase64url(credential.rawId),
      type: credential.type,
      response: {
        authenticatorData: bufferToBase64url(response.authenticatorData),
        clientDataJSON: bufferToBase64url(response.clientDataJSON),
        signature: bufferToBase64url(response.signature),
        userHandle: response.userHandle ? bufferToBase64url(response.userHandle) : null,
      },
    };
  } catch (error: any) {
    console.error('Error authenticating with passkey:', error);
    throw new Error(`Failed to authenticate with passkey: ${error.message}`);
  }
}

// Get user-friendly error messages
export function getWebAuthnErrorMessage(error: Error): string {
  const message = error.message.toLowerCase();
  
  if (message.includes('not allowed')) {
    return 'Operation was cancelled or not allowed. Please try again.';
  }
  
  if (message.includes('timeout')) {
    return 'Operation timed out. Please try again.';
  }
  
  if (message.includes('not supported')) {
    return 'Your browser does not support passkeys. Please use a modern browser.';
  }
  
  if (message.includes('security')) {
    return 'Security requirements not met. Please ensure you are on a secure connection.';
  }
  
  return `Passkey error: ${error.message}`;
}