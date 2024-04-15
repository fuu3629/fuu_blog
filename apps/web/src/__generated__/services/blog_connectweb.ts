// @generated by protoc-gen-connect-web v0.11.0 with parameter "target=ts,import_extension=none"
// @generated from file blog.proto (package blog, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { Blog, Blogs, CreateUserRequest, GetBlogByIdRequest, GetBlogByUserRequest, GetMembersResponse, LoginRequest, PostBlog, Token } from "./blog_pb";
import { Empty, MethodKind } from "@bufbuild/protobuf";

/**
 * @generated from service blog.BlogService
 */
export const BlogService = {
  typeName: "blog.BlogService",
  methods: {
    /**
     * @generated from rpc blog.BlogService.createUser
     */
    createUser: {
      name: "createUser",
      I: CreateUserRequest,
      O: Token,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.deleteUser
     */
    deleteUser: {
      name: "deleteUser",
      I: Empty,
      O: Empty,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.login
     */
    login: {
      name: "login",
      I: LoginRequest,
      O: Token,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.getMembers
     */
    getMembers: {
      name: "getMembers",
      I: Empty,
      O: GetMembersResponse,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.getBlogByUser
     */
    getBlogByUser: {
      name: "getBlogByUser",
      I: GetBlogByUserRequest,
      O: Blogs,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.getBlogById
     */
    getBlogById: {
      name: "getBlogById",
      I: GetBlogByIdRequest,
      O: Blog,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc blog.BlogService.postBlog
     */
    postBlog: {
      name: "postBlog",
      I: PostBlog,
      O: Empty,
      kind: MethodKind.Unary,
    },
  }
} as const;
