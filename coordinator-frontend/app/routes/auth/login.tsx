import { Button, Container, Paper, Text, Title, Alert, TextInput, Stack, Center, Divider, Group } from "@mantine/core";
import {Link, useActionData, Form, redirect, useFetcher, type ActionFunctionArgs} from "react-router";
import { useState, useEffect } from "react";
// Remove coordinatorApi import since we'll use direct fetch for client-side calls
import { commitSession, getSession } from "~/lib/session";
import { isWebAuthnSupported, authenticatePasskey, getWebAuthnErrorMessage } from "~/utils/simplewebauthn";

import {getCoordinatorApi} from "~/services/api";

export function meta() {
  return [
    { title: "Login - Coordinator" },
    { name: "description", content: "Login with your passkey" },
  ];
}

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData();
  
  // Only handle passkey authentication
  const credentialData = formData.get('credential') as string;
  const sessionId = formData.get('sessionId') as string;
  
  if (!credentialData || !sessionId) {
    return { error: 'Invalid passkey authentication data' };
  }
  
  try {
    const credential = JSON.parse(credentialData);
    
    const api = await getCoordinatorApi(request)


    const response = await api.passkey.authFinish({
      session_id: sessionId,
      credential
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
      return { error: response.message ||  'Unknown error'};
    }
  } catch (error: any) {
    console.log(error)
    return { error: error.message || 'Passkey authentication failed' };
  }
}

export default function Login() {
  const actionData = useActionData<typeof action>();
  const fetcher = useFetcher();
  const authStartFetcher = useFetcher();
  const [isPasskeySupported, setIsPasskeySupported] = useState(false);
  const [isPasskeyLoading, setIsPasskeyLoading] = useState(false);
  const [passkeyError, setPasskeyError] = useState<string | null>(null);

  useEffect(() => {
    setIsPasskeySupported(isWebAuthnSupported());
  }, []);

  // Handle auth start response
  useEffect(() => {
    if (authStartFetcher.data && !authStartFetcher.data.message) {
      // We have the auth challenge, now authenticate with passkey
      authenticatePasskey(authStartFetcher.data.request_challenge)
        .then((credential) => {
          // Submit the credential via form
          const formData = new FormData();
          formData.append('credential', JSON.stringify(credential));
          formData.append('sessionId', authStartFetcher.data.session_id);
          
          fetcher.submit(formData, { method: 'post' });
        })
        .catch((error) => {
          console.error('Passkey authentication error:', error);
          setPasskeyError(getWebAuthnErrorMessage(error));
          setIsPasskeyLoading(false);
        });
    } else if (authStartFetcher.data?.message) {
      // Error from auth start
      setPasskeyError(authStartFetcher.data.message);
      setIsPasskeyLoading(false);
    }
  }, [authStartFetcher.data]);

  const handlePasskeyLogin = async () => {
    setIsPasskeyLoading(true);
    setPasskeyError(null);

    // Start passkey authentication using fetcher
    await authStartFetcher.submit(
        JSON.stringify({}),
        {
          method: 'POST',
          action: '/passkey/auth/start',
          encType: 'application/json',
        }
    );
  };

  if (!isPasskeySupported) {
    return (
      <Container size="sm" mt="xl">
        <Center>
          <Paper withBorder shadow="md" p="xl" radius="md" w="100%" maw={400}>
            <Stack gap="lg">
              <Stack gap="xs" ta="center">
                <Title order={2}>Passkey Required</Title>
                <Text size="sm" c="dimmed">
                  This system requires passkey authentication
                </Text>
              </Stack>
              
              <Alert color="orange" title="Passkeys not supported">
                Your browser doesn't support passkeys. Please use a modern browser like Chrome, Firefox, Safari, or Edge.
              </Alert>
              
              <Button component={Link} to="/register" size="lg" fullWidth variant="outline">
                Create Account
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
        <Paper withBorder shadow="md" p="xl" radius="md" w="100%" maw={400}>
          <Stack gap="lg">
            <Stack gap="xs" ta="center">
              <Title order={2}>Login</Title>
              <Text size="sm" c="dimmed">
                Use your passkey to securely login
              </Text>
            </Stack>

            
            <Button 
              size="lg"
              fullWidth
              variant="filled"
              onClick={handlePasskeyLogin}
              loading={isPasskeyLoading || fetcher.state === 'submitting'}
              disabled={isPasskeyLoading || fetcher.state === 'submitting'}
            >
              {isPasskeyLoading ? 'Authenticating...' : 'Login with Passkey'}
            </Button>
            
            {(passkeyError || actionData?.error || fetcher.data?.error) && (
              <Alert color="red">
                {passkeyError || actionData?.error || fetcher.data?.error}
              </Alert>
            )}
            
            <Text size="sm" c="dimmed" ta="center">
              Don't have an account?{' '}
              <Text component={Link} to="/register" c="blue" td="underline">
                Create one here
              </Text>
            </Text>
          </Stack>
        </Paper>
      </Center>
    </Container>
  );
}