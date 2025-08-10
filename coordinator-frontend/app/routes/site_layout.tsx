import {ActionIcon, Badge, Box, Container, Group, Title} from "@mantine/core";
import {TbLogout, TbPlus, TbServerBolt} from "react-icons/tb";
import {modals} from "@mantine/modals";
import {type LoaderFunctionArgs, Outlet, useLoaderData, useNavigate, useSubmit} from "react-router";
import {isAuthenticated} from "~/utils/auth-utils";

export function loader(args: LoaderFunctionArgs) {
    return isAuthenticated(args);
}

export default function AppLayout() {
    const authenticated = useLoaderData<typeof loader>();

    const navigate = useNavigate()
    const handleLogout = async () => {
        navigate('/logout');
    };

    return <Box style={{ minHeight: '100vh', backgroundColor: '#fafafa' }}>
        <Box bg="white" style={{ borderBottom: '1px solid #e5e7eb' }}>
            <Container size="xl" py="md">
                <Group justify="space-between">
                    <Group gap="sm">
                        <TbServerBolt size={24} color="var(--mantine-color-blue-6)" />
                        <Title order={2} c="#111827">Sessio Coordinator</Title>
                    </Group>
                    {authenticated && (
                        <Group gap="sm">
                            <ActionIcon variant="subtle" onClick={handleLogout} color="gray">
                                <TbLogout size={18} />
                            </ActionIcon>
                        </Group>
                    )}
                </Group>
            </Container>
        </Box>
        <Outlet/>
    </Box>
}