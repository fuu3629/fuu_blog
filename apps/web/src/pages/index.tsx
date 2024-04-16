import { useEffect, useState } from "react";
import { clientProvider } from "./api/ClientProvider";
import { BlogPreview, Member } from "@/__generated__/services/blog_pb";
import { BlogCard } from "@/components/UserId/BlogCard";
import { Box, Flex, VStack } from "@chakra-ui/react";
import { MemberList } from "@/components/Index/MemberList/memberList";

export default function Home() {
  const [blogs, setBlogs] = useState<BlogPreview[]>([]);
  const [members, setMembers] = useState<Member[]>([]);
  const client = clientProvider();

  useEffect(() => {
    client.getMembers({}).then((res) => {
      setMembers(res.members);
      const ids = res.members.map((member) => member.userId);
      client.getBlogByUser({ ids: ids }).then((res) => {
        setBlogs(res.blogs);
      });
    });
  }, []);
  return (
    <VStack w="100%">
      <Box w="63vw">
        <Box mb={4}>
          <MemberList members={members} />
        </Box>

        <Flex
          w="63vw"
          p={0}
          gap={"3vw"}
          flexDir={"row"}
          flexWrap={"wrap"}
          alignContent={"flex-start"}
        >
          {blogs.map((blog) => (
            <BlogCard w="30vw" h="224px" blog={blog} userId={blog.userId} />
          ))}
        </Flex>
      </Box>
    </VStack>
  );
}
