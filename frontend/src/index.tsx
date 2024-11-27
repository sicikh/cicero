import { MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import { RouterProvider } from "@tanstack/react-router";
import React from "react";
import ReactDOM from "react-dom/client";
import "./global.css";
import AuthProvider, { useAuth } from "./hooks/AuthProvider.tsx";
import { router } from "./router.tsx";

const InnerApp: React.FC = () => {
  const auth = useAuth();
  return <RouterProvider router={router} context={{ auth }} />;
};

// biome-ignore lint/style/noNonNullAssertion: should always be present
ReactDOM.createRoot(document.querySelector("#root")!).render(
  <React.StrictMode>
    <AuthProvider>
      <MantineProvider>
        <InnerApp />
      </MantineProvider>
    </AuthProvider>
  </React.StrictMode>,
);
