import { useContext, useEffect, useState } from "react";
import { useClient } from "../api/ClientProvider";
import { CokiesContext } from "../api/CokiesContext";
import { useRouter } from "next/router";
import { BlogCard } from "@/components/UserId/BlogCard/BlogCard";
import { Box, HStack, VStack } from "@chakra-ui/react";
import { BlogService } from "@/__generated__/services/blog_connectweb";
import { Blog } from "@/__generated__/services/blog_pb";

export default function Index() {
  const token = useContext(CokiesContext);
  const router = useRouter();
  const id = router.query.userId as string;
  const client = useClient(BlogService);
  const [blogs, setBlogs] = useState<Blog[]>();

  useEffect(() => {
    const res = client.getBlogByUser({ ids: [id] }).then((res) => {
      setBlogs(res.blogs);
    });
  }, []);

  return (
    <VStack w="100%">
      <HStack my={8}>
        <Box bg="gray.600" w="20vw" h="300px" borderRadius={16}>
          profile
        </Box>
        <VStack>
          <Box w="45vw">
            {blogs?.map((blog) => (
              <BlogCard userId={id} blog={blog} />
            ))}
          </Box>
        </VStack>
      </HStack>
    </VStack>
  );
}
