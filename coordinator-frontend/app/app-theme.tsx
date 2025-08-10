import {
  createTheme,
  DEFAULT_THEME,
  MantineProvider,
  type MantineProviderProps,
} from "@mantine/core";
import { ModalsProvider } from "@mantine/modals";
import { AddDeviceModal, EditDeviceModal } from "~/components/DeviceModals";

export const appTheme = createTheme({
  colors: {
    brand: DEFAULT_THEME.colors.blue,
  },
  primaryColor: "brand",
})

export function AppTheme({ children, theme = appTheme,...props }: MantineProviderProps) {
  return (
    <MantineProvider theme={theme} {...props}>
      <ModalsProvider
        modals={{
          addDevice: AddDeviceModal,
          editDevice: EditDeviceModal,
        }}
      >
        {children}
      </ModalsProvider>
    </MantineProvider>
  )
}
