import { useState } from 'react';
import { 
  Stack, 
  Text, 
  TextInput, 
  Button, 
  Group, 
  TagsInput, 
  Tabs,
  Code,
  Box,
  Select,
  Switch,
  CopyButton
} from '@mantine/core';
import { useForm } from '@mantine/form';
import {useSubmit, useRevalidator, useFetcher} from 'react-router';
import { modals, type ContextModalProps } from '@mantine/modals';
import type { DeviceWithCategories, DeviceWithCategory, Category } from '~/services/api';

// Add Device Modal
export interface AddDeviceModalProps {
  categories: Category[];
  coordinatorUrl: string;
}

export const AddDeviceModal = ({ 
  context, 
  id, 
  innerProps 
}: ContextModalProps<AddDeviceModalProps>) => {
  const [selectedOsTab, setSelectedOsTab] = useState<string>("linux");
  const [installKey, setInstallKey] = useState<string | null>(null);
  const [userModeInstall, setUserModeInstall] = useState<boolean>(false);
  const [lastFormData, setLastFormData] = useState<{deviceId: string, deviceType: string} | null>(null);
  const fetcher = useFetcher()
  const revalidator = useRevalidator();

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      deviceId: '',
      deviceType: 'server',
      categories: [] as string[],
    },
    validate: {
      deviceId: (value) => (!value.trim() ? 'Device ID is required' : null),
    },
  });

  const generateInstallCommand = (installKey: string, deviceId: string, deviceType: string, osType: string) => {
    // These URLs can be easily modified later
    const version = import.meta.env.VITE_APP_VERSION
    const INSTALL_SCRIPT_URLS = {
      linux: {
        server: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-server.sh`,
        client: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-client.sh`
      },
      macos: {
        server: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-server-macos.sh`,
        client: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-client-macos.sh`
      },
      windows: {
        server: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-server.ps1`,
        client: `https://github.com/0xC0ffee1/sessio/releases/download/${version}/install-client.ps1`
      }
    };

    const scriptUrl = INSTALL_SCRIPT_URLS[osType as keyof typeof INSTALL_SCRIPT_URLS]?.[deviceType as keyof typeof INSTALL_SCRIPT_URLS.linux] || INSTALL_SCRIPT_URLS.linux.server;
    const coordinatorUrl = innerProps.coordinatorUrl;
    const userFlag = userModeInstall ? ' --user' : '';
    
    switch (osType) {
      case 'windows':
        return `Invoke-WebRequest -Uri "${scriptUrl}" -OutFile "install.ps1"; .\\install.ps1 -InstallKey "${installKey}" -DeviceId "${deviceId}" -Coordinator "${coordinatorUrl}"${userModeInstall ? ' -User' : ''}`;
      case 'macos':
      case 'linux':
      default:
        return `curl -sSL ${scriptUrl} | ${userModeInstall ? 'bash' : 'sudo bash'} -s -- --install-key ${installKey} --coordinator '${coordinatorUrl}'${userFlag}`;
    }
  };

  const handleSubmit = (values: typeof form.values) => {
    const formData = new FormData();
    formData.append('intent', 'createInstallKey');
    formData.append('deviceId', values.deviceId);
    formData.append('deviceType', values.deviceType);
    formData.append('categories', JSON.stringify(values.categories));

    // Store form data for install command generation
    setLastFormData({
      deviceId: values.deviceId,
      deviceType: values.deviceType
    });

    // Use useSubmit hook instead of fetch
    fetcher.submit(formData, { method: 'POST', action: "/dashboard" });
  };

  const handleClose = () => {
    context.closeModal(id);
    revalidator.revalidate();
  };

  // Check for install key from fetcher data
  const currentInstallKey = installKey || fetcher.data?.installKey;

  const cmdLinux = generateInstallCommand(currentInstallKey, lastFormData?.deviceId || form.getValues().deviceId, lastFormData?.deviceType || form.getValues().deviceType, "linux")
  const cmdWindows = generateInstallCommand(currentInstallKey, lastFormData?.deviceId || form.getValues().deviceId, lastFormData?.deviceType || form.getValues().deviceType, "windows")
  const cmdMacos = generateInstallCommand(currentInstallKey, lastFormData?.deviceId || form.getValues().deviceId, lastFormData?.deviceType || form.getValues().deviceType, "macos")


  return (
    <Stack gap="md">
      {!currentInstallKey ? (
        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack gap="md">
            <TextInput
              label="Device ID"
              placeholder="my-server-1"
              required
              description="A unique identifier for this device"
              key={form.key('deviceId')}
              {...form.getInputProps('deviceId')}
            />
            
            <Select
              label="Device Type"
              data={[
                { value: 'server', label: 'Server' },
                { value: 'client', label: 'Client' },
              ]}
              required
              description="Whether this device will act as a server or client"
              key={form.key('deviceType')}
              {...form.getInputProps('deviceType')}
            />

            <TagsInput
              label="Categories"
              placeholder="Select or create categories"
              data={innerProps.categories.map(c => c.name)}
              description="Select categories or create new ones by typing"
              key={form.key('categories')}
              {...form.getInputProps('categories')}
            />
            
            <Switch
              label="User mode install"
              description="Install without sudo privileges (installs to user directory)"
              checked={userModeInstall}
              onChange={(event) => setUserModeInstall(event.currentTarget.checked)}
            />
            
            <Group justify="flex-end" gap="sm">
              <Button variant="default" onClick={handleClose}>
                Cancel
              </Button>
              <Button type="submit">
                Generate Install Key
              </Button>
            </Group>
          </Stack>
        </form>
      ) : (
        <Stack gap="md">
          <Text size="sm" c="green" fw={500}>
            ✓ Install key generated successfully!
          </Text>
          
          <Switch
            label="User mode install"
            description="Install without sudo privileges (installs to user directory)"
            checked={userModeInstall}
            onChange={(event) => setUserModeInstall(event.currentTarget.checked)}
          />
          
          <Tabs value={selectedOsTab} onChange={(value) => setSelectedOsTab(value || "linux")}>
            <Tabs.List>
              <Tabs.Tab value="linux">Linux</Tabs.Tab>
              <Tabs.Tab value="macos">macOS</Tabs.Tab>
              <Tabs.Tab value="windows">Windows</Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="linux" pt="md">
              <Stack>
                <Text size="sm" >Run this command on your Linux device:</Text>
                <Code block style={{ fontSize: '12px', padding: '12px' }}>
                  {cmdLinux}
                </Code>
                <CopyButton value={cmdLinux}>
                  {({ copied, copy }) => (
                    <Button color={copied ? 'teal' : 'blue'} onClick={copy}>
                      {copied ? 'Copied code' : 'Copy code'}
                    </Button>
                  )}
                </CopyButton>
              </Stack>
            </Tabs.Panel>
          </Tabs>
          
          <Group justify="flex-end">
            <Button onClick={handleClose}>
              Done
            </Button>
          </Group>
        </Stack>
      )}
    </Stack>
  );
};

// Edit Device Modal
export interface EditDeviceModalProps {
  device: DeviceWithCategories;
  categories: Category[];
}

export const EditDeviceModal = ({ 
  context, 
  id, 
  innerProps 
}: ContextModalProps<EditDeviceModalProps>) => {
  const revalidator = useRevalidator();
  const fetcher = useFetcher();

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      categories: innerProps.device.categories.map(category => category.name),
    },
  });

  const handleSubmit = (values: typeof form.values) => {
    const formData = new FormData();
    formData.append('intent', 'updateDevice');
    formData.append('deviceId', innerProps.device.device_id);
    formData.append('categories', JSON.stringify(values.categories));

    // Use useSubmit hook instead of fetch
    fetcher.submit(formData, { method: 'POST', action: "/dashboard" });
    
    context.closeModal(id);
    revalidator.revalidate();
  };

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Stack gap="md">
        <Text size="sm" c="dimmed">
          Editing device: <strong>{innerProps.device.device_id}</strong>
        </Text>
        
        <TagsInput
          label="Categories"
          placeholder="Select or create categories"
          data={innerProps.categories.map(c => c.name)}
          description="Select categories or create new ones by typing"
          key={form.key('categories')}
          {...form.getInputProps('categories')}
        />
        
        <Group justify="flex-end" gap="sm">
          <Button variant="default" onClick={() => context.closeModal(id)}>
            Cancel
          </Button>
          <Button type="submit">
            Save Changes
          </Button>
        </Group>
      </Stack>
    </form>
  );
};