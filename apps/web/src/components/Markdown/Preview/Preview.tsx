import type React from "react";
import remarkGfm from "remark-gfm";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { tomorrow } from "react-syntax-highlighter/dist/cjs/styles/prism";
import "github-markdown-css/github-markdown.css";
import ReactMarkdown from "react-markdown";
import { CodeProps } from "react-markdown/lib/ast-to-react";
import styles from "@/styles/react-markdown.module.css";
import { Box } from "@chakra-ui/react";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import "katex/dist/katex.min.css";

interface Props {
  doc?: string;
  height?: number;
}

export function Preview({ doc, height }: Props) {
  return (
    <Box
      className="preview markdown-body"
      boxShadow="xl"
      pl={"4"}
      pr={"4"}
      pt={"2"}
      pb={"2"}
      mx={"64"}
      height={height ? height : "atuo"}
      bgColor={"white"}
      color={"black"}
    >
      <ReactMarkdown
        remarkPlugins={[remarkGfm, remarkMath]}
        rehypePlugins={[rehypeKatex]}
        className={styles.reactMarkDown}
        components={{
          pre({ node, ...props }) {
            return <pre {...props} />;
          },
          code({
            node,
            inline,
            className,
            children,
            style,
            ...props
          }: CodeProps) {
            const match = /language-(\w+)/.exec(className || "");
            return !inline && match ? (
              <SyntaxHighlighter
                style={tomorrow}
                language={match[1]}
                PreTag="div"
                className="preview-code"
                {...props}
              >
                {String(children).replace(/\n$/, "")}
              </SyntaxHighlighter>
            ) : (
              <code className={className} {...props}>
                {children}
              </code>
            );
          },
        }}
      >
        {doc as string}
      </ReactMarkdown>
    </Box>
  );
}

export default Preview;
