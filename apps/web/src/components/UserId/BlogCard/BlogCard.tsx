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
  Spacer,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import dayjs, { extend } from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
extend(relativeTime);

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
          <Box>{dayjs(blog?.createdAt).format("YYYY年M月D日")}</Box>
        </VStack>
        <Spacer></Spacer>
        {dayjs().diff(dayjs(blog?.createdAt), "month") <= 1 ? (
          <Box h="100%">
            <Center
              bg="blue.400"
              borderRadius={16}
              h={8}
              w={16}
              mt={-4}
              mr={-8}
              textAlign={"center"}
              alignItems={"center"}
            >
              New
            </Center>
          </Box>
        ) : null}
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
