import { useEffect, useState } from "react";
import { useClient } from "./api/ClientProvider";
import { BlogService } from "@/__generated__/services/blog_connectweb";
import { Blog, Member } from "@/__generated__/services/blog_pb";
import { BlogCard } from "@/components/UserId/BlogCard";

export default function Home() {
  const [blogs, setBlogs] = useState<Blog[]>([]);
  const [members, setMembers] = useState<Member[]>([]);
  const client = useClient(BlogService);

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
    <div>
      {members.map((member) => (
        <div>{member.name}</div>
      ))}
      {blogs.map((blog) => (
        <BlogCard blog={blog} userId={blog.userId} />
      ))}
    </div>
  );
}
