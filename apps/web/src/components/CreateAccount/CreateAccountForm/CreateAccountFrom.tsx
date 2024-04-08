import {
  Flex,
  Heading,
  Input,
  Button,
  HStack,
  Text,
  Select,
  FormControl,
  FormErrorMessage,
  Spacer,
} from "@chakra-ui/react";
import { useCreateAccountForm } from "./lib";
import { Dispatch, SetStateAction } from "react";
export interface CreateAccountFormProps {
  setToken: Dispatch<SetStateAction<string>>;
}

export function CreateAccountForm({ setToken }: CreateAccountFormProps) {
  const { register, onSubmit, formState } = useCreateAccountForm(setToken);
  return (
    <Flex direction="column" background="gray.200" padding={12} rounded={6}>
      <form onSubmit={onSubmit}>
        <Heading mb={6}>新規登録</Heading>
        {/* <HStack mb={6}>
          <Text h="100%" w="150px">
            Email
          </Text>
          <Input
            placeholder="sample@sample.com"
            bg="white"
            {...register("email")}
          />
        </HStack> */}
        <HStack mb={6}>
          <Text h="100%" w="150px">
            User Name
          </Text>
          <Input placeholder="sample" bg="white" {...register("userName")} />
        </HStack>
        <HStack mb={6}>
          <Text h="100%" w="150px">
            Password
          </Text>
          <Input placeholder="********" bg="white" {...register("password")} />
        </HStack>
        <HStack mb={6}>
          <Text h="100%" w="150px">
            qiitaName
          </Text>
          <Input placeholder="sample" bg="white" {...register("qiitaName")} />
        </HStack>
        <HStack mb={6}>
          <Text h="100%" w="150px">
            qiita API KEY
          </Text>
          <Input placeholder="sample" bg="white" {...register("qiitaApiKey")} />
        </HStack>
        <HStack>
          <Spacer></Spacer>
          <Button mb={6} colorScheme="teal" type="submit">
            crete New Account
          </Button>
        </HStack>
      </form>
    </Flex>
  );
}
