import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import type { PartialMessage } from "@bufbuild/protobuf";
import { Dispatch, SetStateAction } from "react";
import { setCookie } from "nookies";
import { clientProvider } from "@/pages/api/ClientProvider";
import { CreateUserRequest } from "@/__generated__/services/blog_pb";

export const createAccountFormSchema = z.object({
  userName: z.string().min(1, "User Name must be at least 1 characters"),
  password: z.string().min(8, "Password must be at least 8 characters"),
  qiitaName: z.string(),
  qiitaApiKey: z.string(),
});

export type CreateAccountFormSchemaType = z.infer<
  typeof createAccountFormSchema
>;

export const useCreateAccountForm = (
  setToken: Dispatch<SetStateAction<string>>
) => {
  const { register, handleSubmit, formState } =
    useForm<CreateAccountFormSchemaType>({
      resolver: zodResolver(createAccountFormSchema),
    });
  const onSubmit = async (data: CreateAccountFormSchemaType) => {
    const client = clientProvider();
    const req: PartialMessage<CreateUserRequest> = {
      name: data.userName,
      password: data.password,
      QiitaId: data.qiitaName,
      QiitaApiKey: data.qiitaApiKey,
    };
    const res = await client.createUser(req);
    setToken(res.token);
    setCookie(null, "auth", res.token, {
      maxAge: 60 * 60,
      path: "/",
    });
    //TODO
    window.location.href = "/";
  };
  return { register, onSubmit: handleSubmit(onSubmit), formState };
};
