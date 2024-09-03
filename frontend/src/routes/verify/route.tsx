import { Alert, Container, Loader, Text } from "@mantine/core";
import { createFileRoute, redirect, useSearch } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import type React from "react";
import { appAxios } from "../../api/config.ts";

interface ValidateSearch {
  token: string;
}

const Page: React.FC = () => {
  const search = useSearch({ from: "/verify" });
  const [isVerified, setIsVerified] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const verifyToken = async () => {
      try {
        const res = await appAxios.post("/auth/verify", {
          token: search.token,
        });
        if (res.status === 200) {
          setIsVerified(true);
        } else {
          setIsVerified(false);
          setError("Верификация не удалась.");
        }
      } catch (err) {
        setIsVerified(false);
        setError("Верификация не удалась.");
      }
    };

    verifyToken();
  }, [search]);

  return (
    <Container>
      {isVerified === null ? (
        <Loader />
      ) : isVerified ? (
        <Alert title="Верификация прошла успешно" color="green">
          <Text>
            Ваш аккаунт был успешно верифицирован! Вы можете закрыть это окно и
            войти в свой аккаунт.
          </Text>
        </Alert>
      ) : (
        <Alert title="Ошибка" color="red">
          <Text>{error}</Text>
        </Alert>
      )}
    </Container>
  );
};

export const Route = createFileRoute("/verify")({
  component: Page,
  validateSearch: (search: Record<string, unknown>): ValidateSearch => {
    return {
      token: (search.token as string) || "",
    };
  },
  beforeLoad: ({ context, location, search }) => {
    if (context.auth.isAuthenticated) {
      throw redirect({
        to: "/",
        search: {
          redirect: location.href,
        },
      });
    }
    if (search.token === "") {
      throw redirect({
        to: "/login",
        search: {
          redirect: location.href,
        },
      });
    }
  },
});
