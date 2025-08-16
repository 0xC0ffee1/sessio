import { useEffect, useState, useCallback, useMemo } from "react";

import {
  Button,
  Container,
  Text,
  Title,
  Alert,
  TextInput,
  Stack,
  Group,
  Badge,
  Code,
  ActionIcon,
  Center,
  Loader,
  Box,
  Card,
  Chip,
  Menu,
  TagsInput,
  Tabs, ButtonGroup
} from "@mantine/core";
import { modals } from "@mantine/modals";
import {
  useNavigate,
  useLoaderData,
  useRevalidator,
  useActionData,
  Form,
  useFetcher,
  useSubmit,
  redirect
} from "react-router";
import {
  TbCopy,
  TbLogout,
  TbRefresh,
  TbPlus,
  TbDevices,
  TbTrash,
  TbSignature,
  TbEdit,
  TbDots,
  TbDotsVertical, TbServerBolt, TbServer
} from "react-icons/tb";
import {
  getCoordinatorApi,
  type DeviceWithCategories,
  type DeviceWithCategory,
  type Category,
  type Device,
  type ApiError
} from "~/services/api";
import { destroySession, getSession, getJwtToken } from "~/lib/session";
import { authenticatePasskey } from '~/utils/simplewebauthn';
import type { Route } from "./+types/dashboard";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Dashboard - Coordinator" },
    { name: "description", content: "Manage your devices" },
  ];
}

export async function loader({ request }: Route.LoaderArgs) {
  const coordinatorUrl = process.env.COORDINATOR_URL;
  try {
    const coordinatorApi = await getCoordinatorApi(request);
    const devicesResponse = await coordinatorApi.getDevices({});
    return {
      devices: devicesResponse.devices || [],
      categories: devicesResponse.categories || [],
      coordinatorUrl
    };
  } catch (error) {
    if((error as ApiError).status == 401) {
      return redirect("/login")
    }
    console.error('Failed to load devices:', error);
  }
}

export async function action({ request }: Route.ActionArgs) {
  const formData = await request.formData();
  const intent = formData.get('intent');
  
  if (intent === 'logout') {
    const session = await getSession(request);
    const clearCookieHeader = await destroySession(session);
    return new Response(null, {
      status: 302,
      headers: {
        Location: '/login',
        'Set-Cookie': clearCookieHeader,
      },
    });
  }
  
  if (intent === 'createInstallKey') {
    const deviceId = formData.get('deviceId') as string;
    const categories = formData.get('categories') as string;
    
    if (!deviceId) {
      return { error: 'Missing device ID' };
    }
    
    try {
      const coordinatorApi = await getCoordinatorApi(request);
      const categoryList = categories ? JSON.parse(categories) : [];
      const response = await coordinatorApi.createDeviceInstallKey({
        device_id: deviceId,
        categories: categoryList,
      });
      
      return { installKey: response.install_key };
    } catch (error: any) {
      return { error: error.message || 'Failed to create install key' };
    }
  }
  
  if (intent === 'deleteDevice') {
    const deviceId = formData.get('deviceId') as string;
    
    if (!deviceId ) {
      return { error: 'Missing device ID' };
    }
    
    try {
      const coordinatorApi = await getCoordinatorApi(request);
      const response = await coordinatorApi.deleteDevice({
        device_id: deviceId,
      });
      
      if (response.success) {
        return { success: response.message };
      } else {
        return { error: response.message };
      }
    } catch (error: any) {
      return { error: error.message || 'Failed to delete device' };
    }
  }

  if (intent === 'updateDevice') {
    const deviceId = formData.get('deviceId') as string;
    const osName = formData.get('osName') as string;
    const categories = formData.get('categories') as string;
    
    if (!deviceId) {
      return { error: 'Missing device ID' };
    }
    
    try {
      const coordinatorApi = await getCoordinatorApi(request);
      const categoryList = categories ? JSON.parse(categories) : [];
      const response = await coordinatorApi.updateDevice({
        device_id: deviceId,
        os_name: osName || undefined,
        categories: categoryList,
      });
      
      if (response.success) {
        return { success: response.message };
      } else {
        return { error: response.message };
      }
    } catch (error: any) {
      return { error: error.message || 'Failed to update device' };
    }
  }
  
  if (intent === 'signDeviceStart') {
    const deviceId = formData.get('deviceId') as string;
    
    if (!deviceId) {
      return { error: 'Missing device ID' };
    }
    
    try {
      const coordinatorApi = await getCoordinatorApi(request);
      const response = await coordinatorApi.deviceSign.start({
        device_id: deviceId,
      });
      
      return { 
        signDeviceSession: {
          sessionId: response.session_id,
          challenge: response.request_challenge,
          deviceInfo: response.device_info
        }
      };
    } catch (error: any) {
      return { error: error.message || 'Failed to start device signing' };
    }
  }
  
  if (intent === 'signDeviceFinish') {
    const sessionId = formData.get('sessionId') as string;
    const credential = formData.get('credential') as string;
    
    if (!sessionId || !credential) {
      return { error: 'Missing session ID or credential' };
    }
    
    try {
      const coordinatorApi = await getCoordinatorApi(request);
      const response = await coordinatorApi.deviceSign.finish({
        session_id: sessionId,
        credential: JSON.parse(credential)
      });
      
      if (response.success) {
        return { success: response.message || 'Device signed successfully' };
      } else {
        return { error: response.message || 'Failed to sign device' };
      }
    } catch (error: any) {
      return { error: error.message || 'Failed to complete device signing' };
    }
  }
  
  return { error: 'Invalid action' };
}

export default function Dashboard() {
  const {devices, categories, coordinatorUrl} = useLoaderData<typeof loader>()!;
  const actionData = useActionData<typeof action>();
  const revalidator = useRevalidator();
  const signStartFetcher = useFetcher();
  const signFinishFetcher = useFetcher();

  const [selectedCategory, setSelectedCategory] = useState<string>("All");
  const [isSigningDevice, setIsSigningDevice] = useState(false);
  const navigate = useNavigate();


  useEffect(() => {
    const interval = setInterval(() => {
      revalidator.revalidate();
    }, 30000); // 30 seconds

    return () => clearInterval(interval);
  }, [revalidator]);

  const submit = useSubmit()


  const copyToClipboard = useCallback((text: string) => {
    navigator.clipboard.writeText(text);
  }, []);

  const isDeviceActive = useCallback((lastSeenAt: string) => {
    const lastSeen = new Date(lastSeenAt);
    const now = new Date();
    const diffInMinutes = (now.getTime() - lastSeen.getTime()) / (1000 * 60);
    return diffInMinutes <= 2;
  }, []);

  const handleDeleteClick = useCallback((device: DeviceWithCategories) => {
    modals.openConfirmModal({
      title: 'Delete Device',
      children: (
        <Text size="sm">
          Are you sure you want to delete device <strong>{device.device_id}</strong>? This action cannot be undone.
        </Text>
      ),
      labels: { confirm: 'Delete', cancel: 'Cancel' },
      confirmProps: { color: 'red' },
      onConfirm: () => {
        const formData = new FormData();
        formData.append('intent', 'deleteDevice');
        formData.append('deviceId', device.device_id);
        
        // Use useSubmit hook instead of fetch
        submit(formData, { method: 'POST', navigate: false });
      },
    });
  }, [submit]);

  const handleEditClick = useCallback((device: DeviceWithCategories) => {
    modals.openContextModal({
      modal: 'editDevice',
      title: 'Edit Device',
      innerProps: {
        device,
        categories,
      },
    });
  }, [categories]);

  const handleSignDeviceClick = useCallback((device: Device) => {
    modals.openConfirmModal({
      title: 'Sign Device',
      children: (
        <Text size="sm">
          Are you sure you want to sign device <strong>{device.device_id}</strong> with your passkey? 
          This will authorize the device for use.
        </Text>
      ),
      labels: { confirm: 'Sign with Passkey', cancel: 'Cancel' },
      confirmProps: { color: 'blue' },
      onConfirm: () => {
        setIsSigningDevice(true);
        
        // Start device signing using fetcher
        signStartFetcher.submit(
          JSON.stringify({
            device_id: device.device_id
          }),
          {
            method: 'POST',
            action: '/sign/start',
            encType: 'application/json',
          }
        );
      },
    });
  }, [signStartFetcher]);

  // Handle sign start response
  useEffect(() => {
    if (signStartFetcher.data && isSigningDevice && !signStartFetcher.data.message && signStartFetcher.data.request_challenge) {
      // We have the signing challenge, now authenticate with passkey
      authenticatePasskey(signStartFetcher.data.request_challenge)
        .then((credential) => {
          // Submit the credential to finish signing
          signFinishFetcher.submit(
            JSON.stringify({
              session_id: signStartFetcher.data.session_id,
              credential: credential
            }),
            {
              method: 'POST',
              action: '/sign/finish',
              encType: 'application/json',
            }
          );
        })
        .catch((error) => {
          console.error('Passkey authentication error:', error);
          setIsSigningDevice(false);
          // TODO: Show error in UI
        });
    } else if (signStartFetcher.data?.message && isSigningDevice) {
      // Error from sign start
      console.error('Device sign start error:', signStartFetcher.data.message);
      setIsSigningDevice(false);
      // TODO: Show error in UI
    }
  }, [signStartFetcher.data, isSigningDevice]);

  // Handle sign finish response
  useEffect(() => {
    if (signFinishFetcher.data && isSigningDevice) {
      if (signFinishFetcher.data.success) {
        // Success! Reload devices and reset state
        revalidator.revalidate();
      } else {
        // Error from sign finish
        console.error('Device sign finish error:', signFinishFetcher.data.message);
      }
      setIsSigningDevice(false);
    }
  }, [signFinishFetcher.data, revalidator, isSigningDevice]);


  const filteredDevices = useMemo(() => {
    let filtered = devices;
    
    // Filter by category
    if (selectedCategory !== "All") {
      filtered = filtered.filter(device =>
        device.categories.some(category => category.name === selectedCategory)
      );
    }
    
    return filtered;
  }, [devices, selectedCategory]);


  return (
      <Container size="xl" py="xl">
        <Stack gap="md">
          {/* Devices Section */}
          <Group justify="space-between">
            <Group gap="sm">
              <Title order={3} c="#111827">Devices</Title>
              <Badge variant="light" color="gray" size="sm">
                {filteredDevices.length}
              </Badge>
            </Group>
            <Button
                size="sm"
                leftSection={<TbPlus size={16} />}
                onClick={() => modals.openContextModal({
                  modal: 'addDevice',
                  title: 'Add New Device',
                  innerProps: {
                    categories,
                    actionData,
                    coordinatorUrl
                  },
                })}
            >
              Add Device
            </Button>
          </Group>
          <Card shadow="sm" padding="lg" radius="md" withBorder bg="white">
            <Stack gap="lg">

              <Group justify="space-between">
                {/* Category Filter Chips */}
                <Chip.Group multiple={false} value={selectedCategory} onChange={setSelectedCategory}>
                  <Group gap="xs">
                    <Chip value="All" variant="filled">All</Chip>
                    {categories.map((category) => (
                        <Chip key={category.id} value={category.name} variant="filled">
                          {category.name}
                        </Chip>
                    ))}
                  </Group>
                </Chip.Group>
                <ActionIcon
                    variant="light"
                    onClick={() => {revalidator.revalidate()}}
                    disabled={revalidator.state === 'loading'}
                    color="gray"
                >
                  {revalidator.state === 'loading' ? <Loader size={16} /> : <TbRefresh size={16} />}
                </ActionIcon>
              </Group>



              {revalidator.state === 'loading' ? (
                  <Center py="xl">
                    <Stack gap="sm" align="center">
                      <Loader size="md" />
                      <Text size="sm" c="dimmed">Loading devices...</Text>
                    </Stack>
                  </Center>
              ) : filteredDevices.length === 0 ? (
                  <Center py="xl">
                    <Stack gap="md" align="center">
                      <TbDevices size={48} color="#9ca3af" />
                      <Stack gap="xs" align="center">
                        <Text fw={500} c="dimmed">
                          No devices found
                        </Text>
                        <Text size="sm" c="dimmed" ta="center">
                          Click "Add Device" to create an install key for your first device
                        </Text>
                      </Stack>
                    </Stack>
                  </Center>
              ) : (
                  <Stack>
                    {filteredDevices.map((device, index) => (
                        <Box
                            key={device.id}
                            style={{
                              borderBottom: index < filteredDevices.length - 1 ? '1px solid #f3f4f6' : 'none',
                              paddingBottom: '5px',
                              backgroundColor: 'white'
                            }}
                        >
                          <Group justify="space-between">
                            <Group gap="md">
                              <Box
                                  w={40}
                                  h={40}
                                  style={{
                                    borderRadius: '8px',
                                    backgroundColor: isDeviceActive(device.last_seen_at) ? '#10b981' : '#6b7280',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center'
                                  }}
                              >
                                <TbDevices size={20} color="white" />
                              </Box>
                              <Stack gap={2} style={{ flex: 1 }}>
                                <Group gap="sm">
                                  <Text fw={600} size="sm" c="#111827">{device.device_id}</Text>

                                  {isDeviceActive(device.last_seen_at) && (
                                      <Badge color="green" size="xs" variant="light">
                                        Online
                                      </Badge>
                                  )}
                                  {device.signature ? (
                                      <Badge color="blue" size="xs" variant="light">
                                        Signed
                                      </Badge>
                                  ) : (
                                      <Badge color="orange" size="xs" variant="light">
                                        Unsigned
                                      </Badge>
                                  )}
                                </Group>
                                <Text size="xs" c="dimmed">
                                  {device.os_name || 'Unknown OS'} 
                                  {device.version && ` • ${device.version} • `}
                                  Last seen {new Date(device.last_seen_at).toLocaleString()}
                                </Text>
                                {device.categories.length > 0 && (
                                    <Group gap="xs">
                                      {device.categories.map((category) => (
                                          <Badge key={category.id} color="grape" size="xs" variant="light">
                                            {category.name}
                                          </Badge>
                                      ))}
                                    </Group>
                                )}
                              </Stack>
                            </Group>
                            <Menu shadow="md" width={200}>
                              <Menu.Target>
                                <ActionIcon variant="subtle" color="gray" size="sm">
                                  <TbDotsVertical size={16} />
                                </ActionIcon>
                              </Menu.Target>
                              <Menu.Dropdown>
                                <Menu.Item leftSection={<TbEdit size={14} />} onClick={() => handleEditClick(device)}>
                                  Edit device
                                </Menu.Item>
                                {device.public_key && (
                                    <Menu.Item leftSection={<TbCopy size={14} />} onClick={() => copyToClipboard(device.public_key!)}>
                                      Copy public key
                                    </Menu.Item>
                                )}
                                {!device.signature && device.public_key && (
                                    <Menu.Item leftSection={<TbSignature size={14} />} onClick={() => handleSignDeviceClick(device)}>
                                      Sign with passkey
                                    </Menu.Item>
                                )}
                                <Menu.Divider />
                                <Menu.Item leftSection={<TbTrash size={14} />} color="red" onClick={() => handleDeleteClick(device)}>
                                  Delete device
                                </Menu.Item>
                              </Menu.Dropdown>
                            </Menu>
                          </Group>
                        </Box>
                    ))}
                  </Stack>
              )}
            </Stack>
          </Card>

        </Stack>
      </Container>
  );
}
