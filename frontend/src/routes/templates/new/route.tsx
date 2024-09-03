import { createFileRoute, redirect } from "@tanstack/react-router";
import type React from "react";
import { useState } from "react";
import { Button, TextInput, Container } from "@mantine/core";
import { useAuth } from "../../../hooks/AuthProvider.tsx";
import { TemplatesApi } from "../-api/templates.api.ts";

interface FormValues {
  name: string;
  description: string;
}

const Page: React.FC = () => {
  const { isAuthenticated } = useAuth();
  
  return (
    <Container>
    
    </Container>
  );
};

export const Route = createFileRoute("/templates/new")({
  component: Page,
  beforeLoad: ({ context }) => {
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: "/login",
        search: {
          redirect: "/templates/new",
        },
      });
    }
  },
});