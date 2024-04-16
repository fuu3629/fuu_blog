import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import type { PartialMessage } from "@bufbuild/protobuf";
import { setCookie } from "nookies";
import { clientProvider } from "@/pages/api/ClientProvider";
import { BlogService } from "@/__generated__/services/blog_connectweb";
import { LoginRequest } from "@/__generated__/services/blog_pb";

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
    const client = clientProvider();
    const req: PartialMessage<LoginRequest> = {
      userId: data.name,
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
