import { clientProvider } from "@/pages/api/ClientProvider";
import { useRouter } from "next/router";
import { useContext, useEffect, useState } from "react";
import { CokiesContext } from "@/pages/api/CokiesContext";
import { Blog } from "@/__generated__/services/blog_pb";

export default function Index() {
  const token = useContext(CokiesContext);
  const router = useRouter();
  const { userId, blogId } = router.query;
  const client = clientProvider();
  const [blog, setBlog] = useState<Blog>();

  useEffect(() => {
    const res = client
      .getBlogById({ id: BigInt(blogId as string) })
      .then((res) => {
        setBlog(res);
      });
  });
  return (
    <div>
      <h1>{blog?.title}</h1>
      <div>{blog?.body}</div>
    </div>
  );
}
