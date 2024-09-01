import { MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import { RouterProvider } from "@tanstack/react-router";
import React from "react";
import ReactDOM from "react-dom/client";
import "./global.css";
import { router } from "./router.tsx";

// biome-ignore lint/style/noNonNullAssertion: should always be present
ReactDOM.createRoot(document.querySelector("#root")!).render(
  <React.StrictMode>
    <MantineProvider >
      <RouterProvider router={router} />
    </MantineProvider>
  </React.StrictMode>,
);
