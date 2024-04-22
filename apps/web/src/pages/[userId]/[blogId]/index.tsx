import { clientProvider } from "@/pages/api/ClientProvider";
import { useRouter } from "next/router";
import { useContext, useEffect, useState } from "react";
import { CokiesContext } from "@/pages/api/CokiesContext";
import { Blog } from "@/__generated__/services/blog_pb";
import { Button } from "@chakra-ui/react";

export default function Index() {
  const token = useContext(CokiesContext);
  const router = useRouter();
  const { userId, blogId } = router.query;
  const client = clientProvider();
  const [blog, setBlog] = useState<Blog>();
  const [summary, setSummary] = useState<string>("");

  useEffect(() => {
    client.getBlogById({ id: BigInt(blogId as string) }).then((res) => {
      setBlog(res);
    });
  });

  const onClickSummary = async () => {
    const stream = client.getSummaryStream({
      blogId: BigInt(blogId as string),
    });
    for await (const delta of stream) {
      console.log(delta.summaryText);
      setSummary((prev) => prev + delta.summaryText);
    }
  };

  return (
    <div>
      <div>{summary}</div>
      <Button onClick={onClickSummary}>aaa</Button>
      <h1>{blog?.title}</h1>
      <div>{blog?.body}</div>
    </div>
  );
}
