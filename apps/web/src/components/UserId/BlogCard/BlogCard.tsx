import { Blog, BlogPreview } from "@/__generated__/services/blog_pb";
import {
  Avatar,
  Badge,
  Box,
  Center,
  Flex,
  HStack,
  Heading,
  Link,
  VStack,
  Text,
} from "@chakra-ui/react";
import { useRouter } from "next/router";

export interface BlogCardProps {
  userId: string;
  blog?: Blog | BlogPreview;
  w?: string;
  h?: string;
}

export function BlogCard({
  userId,
  blog,
  w = "100%",
  h = "100%",
}: BlogCardProps) {
  const router = useRouter();

  return (
    <Box bg="gray.600" px={8} py={4} w={w} borderRadius={16} h={h} gap={4}>
      <HStack h={16}>
        <Avatar m={2}></Avatar>
        <VStack gap={1} alignItems={"flex-start"}>
          <Heading size="sm">{userId}</Heading>
          <Box>{blog?.createdAt}</Box>
        </VStack>
      </HStack>
      <Flex h={16} alignItems={"center"} my={4}>
        <Link
          onClick={async () => {
            await router.push(`/${userId}/${blog?.id}`);
          }}
        >
          <Heading fontSize="22px" noOfLines={2}>
            {blog?.title}
          </Heading>
        </Link>
      </Flex>
      <Flex h="32px" overflowX={"hidden"}>
        {blog?.tags.map((tag) => (
          <Badge key={tag.name} m={1} color="white">
            <Text mt="2px">{tag.name}</Text>
          </Badge>
        ))}
      </Flex>
    </Box>
  );
}
