import { Flex } from "@chakra-ui/react";
import { LoginForm } from "@/components/Login/LoginForm";

export default function Login() {
  return (
    <Flex height="100vh" alignItems="center" justifyContent="center" w="100%">
      <LoginForm></LoginForm>
    </Flex>
  );
}
