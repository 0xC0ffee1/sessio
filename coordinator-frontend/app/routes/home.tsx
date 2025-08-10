import { useEffect } from "react";
import { useNavigate } from "react-router";
import { Container, Title, Text, Button, Group, Paper, Center, Stack } from "@mantine/core";
import { Link } from "react-router";
import type { Route } from "./+types/home";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Coordinator - Device Management" },
    { name: "description", content: "Manage your devices with the coordinator" },
  ];
}

export default function Home() {
  const navigate = useNavigate();

  return (
    <Container size="sm" mt="xl">
      <Center>
        <Paper withBorder shadow="md" p="xl" radius="md" w="100%" maw={400}>
          <Stack gap="lg">
            <Stack gap="xs" ta="center">
              <Title order={1}>Sessio Control Panel</Title>
              <Text c="dimmed">
                Manage your devices and create secure connections
              </Text>
            </Stack>
            
            <Group justify="center" gap="md">
              <Button 
                component={Link} 
                to="/login"
                size="lg"
                variant="filled"
                fullWidth
              >
                Login with Passkey
              </Button>
              
              <Button 
                component={Link} 
                to="/register"
                size="lg"
                variant="outline"
                fullWidth
              >
                Create Account with Passkey
              </Button>
            </Group>
          </Stack>
        </Paper>
      </Center>
    </Container>
  );
}
