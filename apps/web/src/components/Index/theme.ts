import { extendTheme } from "@chakra-ui/react";

// NB: Chakra gives you access to `colorMode` and `theme` in `props`
export const theme = extendTheme({
  styles: {
    global: (props: any) => ({
      "html, body": {
        fontSize: "sm",
        color: props.colorMode === "dark" ? "white" : "gray.600",
        lineHeight: "tall",
        padding: 0,
        margin: 0,
      },
      a: {
        color: props.colorMode === "dark" ? "white" : "gray.600",
        textDecoration: "none",
      },
    }),
  },
});
