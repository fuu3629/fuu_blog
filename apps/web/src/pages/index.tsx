import { useEffect, useState } from "react";
import { clientProvider } from "./api/ClientProvider";
import { BlogPreview, Member } from "@/__generated__/services/blog_pb";
import { BlogCard } from "@/components/UserId/BlogCard";
import { Box, Flex, VStack, Divider, Heading } from "@chakra-ui/react";
import { MemberList } from "@/components/Index/MemberList/memberList";
import ReactPaginate from "react-paginate";
import Pagination from "@/components/Index/Pagenation/pagination";

export default function Home() {
  const [blogs, setBlogs] = useState<BlogPreview[]>([]);
  const [members, setMembers] = useState<Member[]>([]);
  const [page, setPage] = useState(1);
  const [totalCount, setTotalCount] = useState(0);
  const client = clientProvider();

  useEffect(() => {
    client.getMembers({}).then((res) => {
      setMembers(res.members);
      const ids = res.members.map((member) => member.userId);
      client
        .getBlogByUsers({
          ids: ids,
          pagination: {
            page: page,
            pageSize: 10,
            order: 1,
          },
        })
        .then((res) => {
          console.log(res);
          setBlogs(res.blogs);
          setTotalCount(res.totalCount);
        });
    });
  }, [page]);
  return (
    <VStack w="100%">
      <Box w="63vw">
        <Heading>Membres</Heading>
        <Divider borderColor="gray.500" borderBottomWidth={"2px"} />
        <Box mb={12} mt={4}>
          <MemberList members={members} />
        </Box>

        <Heading>Articles</Heading>
        <Divider borderColor="gray.500" borderBottomWidth={"2px"} />
        <Flex
          w="63vw"
          my={8}
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
        <Pagination
          pageSize={10}
          totalCount={totalCount}
          onChange={(page) => {
            setPage(page);
          }}
        ></Pagination>
      </Box>
    </VStack>
  );
}
