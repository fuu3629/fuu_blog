import { Box, Button, Spacer, Text, useColorMode } from "@chakra-ui/react";

export interface AppBarProps {}

export function AppBar({}: AppBarProps) {
  const { colorMode, toggleColorMode } = useColorMode();
  return (
    <Box bg="blue.400" px="24px">
      <Text fontSize={"3xl"} color="white">
        シフト管理アプリ
      </Text>
      <Spacer></Spacer>
      <Button onClick={toggleColorMode}></Button>
    </Box>
  );
}
