import { use, useEffect, useState } from "react";
import { useClient } from "./api/ClientProvider";
import { BlogService } from "../../services/blog_connectweb";
import { Member } from "../../services/blog_pb";

export default function Home() {
  const [blogs, setBlogs] = useState([]);
  const [members, setMembers] = useState<Member[]>([]);
  const client = useClient(BlogService);

  useEffect(() => {
    const res = client.getMembers({}).then((res) => {
      setMembers(res.members);
      const ids = res.members.map((member) => member.id);
      const blog = client.getBlogByUser({ ids: ids }).then((res) => {});
    });
  }, []);
  return (
    <div>
      <h1>Home</h1>
    </div>
  );
}
