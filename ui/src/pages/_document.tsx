import { Html, Head, Main, NextScript } from "next/document";
import InnerHeader from "./_head";

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <InnerHeader />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
