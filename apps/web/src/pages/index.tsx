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
  const [pageInfo, setPageInfo] = useState({
    pageSize: 10,
    totalCount: 0,
  });
  const client = clientProvider();

  useEffect(() => {
    client.getMembers({}).then((res) => {
      setMembers(res.members);
      const ids = res.members.map((member) => member.userId);
      client
        .getBlogByUser({
          ids: ids,
          pagination: {
            page: page,
            pageSize: pageInfo.pageSize,
            order: 1,
          },
        })
        .then((res) => {
          setBlogs(res.blogs);
          setPageInfo({
            pageSize: res.pageInfo?.pagination?.pageSize!,
            totalCount: res.pageInfo?.totalCount!,
          });
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
          pageSize={pageInfo.pageSize}
          totalCount={pageInfo.totalCount}
          onChange={(page) => {
            setPage(page);
          }}
        ></Pagination>
      </Box>
    </VStack>
  );
}
