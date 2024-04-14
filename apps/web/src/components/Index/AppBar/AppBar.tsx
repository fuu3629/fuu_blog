import { Button, Flex, Spacer, Text, useColorMode } from "@chakra-ui/react";

export interface AppBarProps {}

export function AppBar({}: AppBarProps) {
  const { colorMode, toggleColorMode } = useColorMode();
  return (
    <Flex bg="blue.600" px="24px">
      <Text fontSize={"3xl"} color="white">
        Sky Intern Blog
      </Text>
      <Spacer></Spacer>
      <Button onClick={toggleColorMode}></Button>
    </Flex>
  );
}
