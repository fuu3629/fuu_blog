import {
  Avatar,
  Box,
  Flex,
  HStack,
  Heading,
  Link,
  VStack,
} from "@chakra-ui/react";
import { Blog } from "../../../../services/blog_pb";
import { useRouter } from "next/router";

export interface BlogCardProps {
  userId: string;
  blog?: Blog;
  w?: string;
}

export function BlogCard({ userId, blog, w = "100%" }: BlogCardProps) {
  const router = useRouter();

  return (
    <Box bg="gray.600" px={8} py={4} w={w} m={4} borderRadius={16}>
      <HStack>
        <Avatar m={2}></Avatar>
        <VStack h="100%" gap={1} alignItems={"flex-start"}>
          <Heading size="sm">{userId}</Heading>
          <Box>{blog?.createdAt}</Box>
        </VStack>
      </HStack>
      <Link
        onClick={async () => {
          await router.push(`/${userId}/${blog?.id}`);
        }}
      >
        <Heading fontSize="26px">{blog?.title}</Heading>
      </Link>
      <Box>{blog?.tags[0].name}</Box>
    </Box>
  );
}
