import { Member } from "@/__generated__/services/blog_pb";

export interface MemberListProps {
  members: Member[];
}

export function MemberList() {
  return (
    <div>
      <div>member.name</div>
    </div>
  );
}
