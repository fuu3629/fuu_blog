import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import type { PromiseClient } from "@bufbuild/connect";
import { BlogService } from "../../../services/blog_connectweb";
import React from "react";

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
