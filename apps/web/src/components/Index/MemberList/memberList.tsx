import { Member } from "@/__generated__/services/blog_pb";
import { Avatar, Flex, HStack, VStack } from "@chakra-ui/react";
import { useRouter } from "next/router";

export interface MemberListProps {
  members: Member[];
}

export function MemberList({ members }: MemberListProps) {
  const router = useRouter();
  const handleClickMember = async (member: Member) => {
    await router.push(`/${member.userId}`);
  };
  return (
    <>
      <Flex gap={8}>
        {members.map((member) => (
          <VStack onClick={() => handleClickMember(member)}>
            {/* //TODO 画像の対応 */}
            <Avatar />
            <div>{member.name}</div>
          </VStack>
        ))}
      </Flex>
    </>
  );
}
