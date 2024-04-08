import { useState } from "react";
import { Flex } from "@chakra-ui/react";
import { CreateAccountForm } from "@/components/CreateAccount/CreateAccountForm";

export default function CreateNewAccount() {
  const [token, setToken] = useState<string>("");

  return (
    <Flex height="100vh" alignItems="center" justifyContent="center" w="100%">
      <CreateAccountForm setToken={setToken} />
    </Flex>
  );
}
