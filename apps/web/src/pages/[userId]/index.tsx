import { useContext, useEffect, useState } from "react";
import { clientProvider } from "../api/ClientProvider";
import { CokiesContext } from "../api/CokiesContext";
import { useRouter } from "next/router";
import { BlogCard } from "@/components/UserId/BlogCard/BlogCard";
import { Box, Flex, HStack, VStack } from "@chakra-ui/react";
import { Blog, BlogPreview } from "@/__generated__/services/blog_pb";

export default function Index() {
  const token = useContext(CokiesContext);
  const router = useRouter();
  const id = router.query.userId as string;
  const client = clientProvider();
  const [blogs, setBlogs] = useState<BlogPreview[]>();

  useEffect(() => {
    const res = client.getBlogByUser({ ids: [id] }).then((res) => {
      setBlogs(res.blogs);
    });
  }, []);

  return (
    <VStack w="100%">
      <Flex gap={8}>
        <Box bg="gray.600" w="20vw" h="300px" borderRadius={16}>
          profile
        </Box>
        <VStack>
          <VStack w="45vw" gap={8}>
            {blogs?.map((blog) => (
              <BlogCard userId={id} blog={blog} />
            ))}
          </VStack>
        </VStack>
      </Flex>
    </VStack>
  );
}
