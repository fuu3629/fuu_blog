import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import { BlogService } from "@/__generated__/services/blog_connectweb";

const transport = createGrpcWebTransport({
  baseUrl: process.env.NEXT_PUBLIC_BACKEND_URL as string,
});

export function clientProvider() {
  return createPromiseClient(BlogService, transport);
}
