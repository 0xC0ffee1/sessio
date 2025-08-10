import { Button, Container, Paper, Text, Title, Alert, Stack, Center, TextInput } from "@mantine/core";
import {Link, useActionData, Form, redirect, useFetcher, useSubmit, type ActionFunctionArgs} from "react-router";
import { useState, useEffect } from "react";
import { getCoordinatorApi } from "~/services/api";
import {isWebAuthnSupported, createPasskey, getWebAuthnErrorMessage} from "~/utils/simplewebauthn";
import { getSession, commitSession } from "~/lib/session";

export function meta() {
  return [
    { title: "Register - Coordinator" },
    { name: "description", content: "Create a new account with passkey" },
  ];
}

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData();
  const actionType = formData.get('actionType') as string;
  
  if (actionType === 'complete_registration') {
    // Handle passkey registration completion
    const credentialData = formData.get('credential') as string;
    const sessionId = formData.get('sessionId') as string;
    
    if (!credentialData || !sessionId) {
      return { error: 'Invalid passkey registration data' };
    }
    
    try {
      const credential = JSON.parse(credentialData);
      const coordinatorApi = await getCoordinatorApi(request);
      const response = await coordinatorApi.passkey.registerFinish({
        session_id: sessionId,
        credential: credential,
      });
      
      if (response.success && response.jwt_token) {
        // Store only JWT token in session cookie
        const session = await getSession(request);
        session.set('jwt_token', response.jwt_token);

        return redirect('/dashboard', {
          headers: {
            'Set-Cookie': await commitSession(session),
          },
        });
      } else {
        return { 
          error: response.message || 'Failed to complete registration'
        };
      }
    } catch (error: any) {
      return { 
        error: error.message || 'Failed to complete registration'
      };
    }
  }
  
  return { error: 'Invalid action' };
}

export default function Register() {
  const actionData = useActionData<typeof action>();
  const [isPasskeySupported, setIsPasskeySupported] = useState(false);
  const [isRegistering, setIsRegistering] = useState(false);
  const [registrationError, setRegistrationError] = useState<string | null>(null);
  const [username, setUsername] = useState('');
  const [displayName, setDisplayName] = useState('');

  const registerStartFetcher = useFetcher();

  const submit = useSubmit()

  useEffect(() => {
    setIsPasskeySupported(isWebAuthnSupported());
  }, []);

  async function checkRegisterStatus() {
    try {
      const registerStartResponse = registerStartFetcher.data
      // Use browser WebAuthn API to create passkey
      const credential = await createPasskey(registerStartResponse.creation_challenge);

      // Submit form to complete registration via React Router action
      const formData = new FormData();
      formData.append('actionType', 'complete_registration');
      formData.append('credential', JSON.stringify(credential));
      formData.append('sessionId', registerStartResponse.session_id);

      submit(formData, {
        method: "POST"
      })
    }
    catch (error: any) {
      console.error('Passkey registration error:', error);
      setRegistrationError(getWebAuthnErrorMessage(error));
    } finally {
      setIsRegistering(false);
    }
  }

  useEffect(() => {
    if (registerStartFetcher.data && !registerStartFetcher.data.message) {
      checkRegisterStatus()
    }
  }, [registerStartFetcher.data]);

  // Handle single-step passkey registration
  const handlePasskeyRegistration = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!username || !displayName) {
      setRegistrationError('Please fill in username and display name');
      return;
    }
    
    setIsRegistering(true);
    setRegistrationError(null);

    try {
      await registerStartFetcher.submit(
          {
            username: username,
            display_name: displayName,
          },
          {
            method: 'POST',
            action: '/passkey/register/start',
            encType: 'application/json',
          }
      );
    } catch (error: any) {
      console.error('Passkey registration error:', error);
      setRegistrationError(getWebAuthnErrorMessage(error));
    } finally {
      setIsRegistering(false);
    }
  };

  if (!isPasskeySupported) {
    return (
      <Container size="sm" mt="xl">
        <Center>
          <Paper withBorder shadow="md" p="xl" radius="md" w="100%" maw={400}>
            <Stack gap="lg">
              <Stack gap="xs" ta="center">
                <Title order={2}>Passkeys Required</Title>
                <Text size="sm" c="dimmed">
                  This system requires passkey authentication
                </Text>
              </Stack>
              
              <Alert color="orange" title="Passkeys not supported">
                Your browser doesn't support passkeys. Please use a modern browser like Chrome, Firefox, Safari, or Edge to create an account.
              </Alert>
              
              <Button component={Link} to="/" size="lg" fullWidth variant="outline">
                Back to Home
              </Button>
            </Stack>
          </Paper>
        </Center>
      </Container>
    );
  }

  return (
    <Container size="sm" mt="xl">
      <Center>
        <Paper withBorder shadow="md" p="xl" radius="md" w="100%" maw={500}>
          <Stack gap="lg">
            <Stack gap="xs" ta="center">
              <Title order={2}>Create Account with Passkey</Title>
              <Text size="sm" c="dimmed">
                Create your account using a passkey for secure, passwordless authentication.
              </Text>
            </Stack>
            
            <form onSubmit={handlePasskeyRegistration}>
              <Stack gap="md">
                <TextInput
                  label="Username"
                  placeholder="john_doe"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  required
                  description="A username for your passkey (can be anything you prefer)"
                />
                
                <TextInput
                  label="Display Name"
                  placeholder="John Doe"
                  value={displayName}
                  onChange={(e) => setDisplayName(e.target.value)}
                  required
                  description="Your full name or preferred display name"
                />
                
                {(registrationError || actionData?.error) && (
                  <Alert color="red">
                    {registrationError || actionData?.error}
                  </Alert>
                )}
                
                <Button 
                  type="submit" 
                  size="lg"
                  fullWidth
                  loading={isRegistering}
                  disabled={isRegistering || !username || !displayName}
                >
                  {isRegistering ? 'Creating Account & Passkey...' : '🔑 Create Account with Passkey'}
                </Button>
              </Stack>
            </form>
            
            <Text size="sm" c="dimmed" ta="center">
              Already have an account?{' '}
              <Text component={Link} to="/login" c="blue" td="underline">
                Login here
              </Text>
            </Text>
          </Stack>
        </Paper>
      </Center>
    </Container>
  );
}