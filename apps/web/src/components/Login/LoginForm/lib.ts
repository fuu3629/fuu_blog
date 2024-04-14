import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import { BlogService } from "../../../../services/blog_connectweb";
import type { PartialMessage } from "@bufbuild/protobuf";
import { LoginRequest } from "../../../../services/blog_pb";
import { setCookie } from "nookies";

export const loginFormSchema = z.object({
  name: z.string(),
  password: z.string(),
});

export type LoginFormSchemaType = z.infer<typeof loginFormSchema>;

export const useLoginForm = () => {
  const { register, handleSubmit, formState } = useForm<LoginFormSchemaType>({
    resolver: zodResolver(loginFormSchema),
  });
  const onSubmit = async (data: LoginFormSchemaType) => {
    const transport = createGrpcWebTransport({
      baseUrl: "http://127.0.0.1:50052",
    });
    const client = createPromiseClient(BlogService, transport);
    const req: PartialMessage<LoginRequest> = {
      name: data.name,
      password: data.password,
    };
    try {
      const res = await client.login(req);
      setCookie(null, "auth", res.token, {
        maxAge: 60 * 60,
        path: "/",
      });
      window.location.href = "/";
    } catch (e) {
      alert("Login failed");
    }
  };
  return { register, onSubmit: handleSubmit(onSubmit), formState };
};
