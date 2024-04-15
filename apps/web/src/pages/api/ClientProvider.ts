import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import type { PromiseClient } from "@bufbuild/connect";
import React from "react";
import { BlogService } from "@/__generated__/services/blog_connectweb";

const transport = createGrpcWebTransport({
  baseUrl: process.env.NEXT_PUBLIC_BACKEND_URL as string,
});

const client = createPromiseClient(BlogService, transport);
export function useClient<T extends typeof BlogService>(
  service: T
): PromiseClient<T> {
  return React.useMemo(
    () => createPromiseClient(service, transport),
    [service]
  );
}
