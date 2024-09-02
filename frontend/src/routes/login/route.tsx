import { Autocomplete, Button, Loader, PasswordInput } from "@mantine/core";
import { useForm } from "@tanstack/react-form";
import { Link, createFileRoute, redirect } from "@tanstack/react-router";
import type React from "react";
import { useRef, useState } from "react";
import { useAuth } from "../../hooks/AuthProvider.tsx";
import type { LoginDto } from "./-api/dtos/Login.dto.ts";
import styles from "./route.module.css";

interface FormValues extends LoginDto {
  rememberMe: boolean;
}

const Page: React.FC = () => {
  const { login } = useAuth();
  const timeoutRef = useRef<number>(-1);
  const [isLoading, setIsLoading] = useState(false);
  const [emailProviders, setEmailProviders] = useState<string[]>([]);

  const form = useForm<FormValues>({
    defaultValues: {
      email: "",
      password: "",
      rememberMe: false,
    },
    onSubmit: async ({ value }) => {
      const { success } = await login({
        email: value.email,
        password: value.password,
      });

      if (!success) {
        // TODO: correctly show error message, for now just alert
        alert("Неверный логин или пароль");
      }
    },
  });

  return (
    <div id={styles.wrapper}>
      <Link to="/" className={styles["home-button"]}>
        На главную
      </Link>
      <form
        id="login"
        onSubmit={(e) => {
          e.preventDefault();
          form.handleSubmit();
        }}
        className={styles.form}
      >
        <h1>Войти</h1>
        <div className={styles["input-box"]}>
          <form.Field
            name={"email"}
            validators={{
              onChange: ({ value }) =>
                value === "" ? "Поле обязательно для заполнения" : undefined,
            }}
          >
            {(field) => {
              return (
                <Autocomplete
                  variant="unstyled"
                  size="lg"
                  data={emailProviders}
                  withAsterisk={true}
                  value={field.state.value}
                  onChange={(e) => {
                    window.clearTimeout(timeoutRef.current);
                    field.handleChange(e);
                    setEmailProviders([]);

                    if (e.trim().length === 0 || e.includes("@")) {
                      setIsLoading(false);
                    } else {
                      setIsLoading(true);
                      timeoutRef.current = window.setTimeout(() => {
                        setIsLoading(false);
                        setEmailProviders(
                          ["mail.ru", "gmail.com", "outlook.com"].map(
                            (provider) => `${e}@${provider}`,
                          ),
                        );
                      }, 1000);
                    }
                  }}
                  onBlur={field.handleBlur}
                  error={field.state.meta.errors.at(0)}
                  rightSection={isLoading ? <Loader size="1rem" /> : null}
                  placeholder="E-mail"
                />
              );
            }}
          </form.Field>
        </div>
        <div className={styles["input-box"]}>
          <form.Field
            name={"password"}
            validators={{
              onChange: ({ value }) =>
                value === "" ? "Поле обязательно для заполнения" : undefined,
            }}
          >
            {(field) => {
              return (
                <PasswordInput
                  placeholder="Пароль"
                  size="lg"
                  variant="unstyled"
                  withAsterisk={true}
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)}
                  onBlur={field.handleBlur}
                  error={field.state.meta.errors.at(0)}
                />
              );
            }}
          </form.Field>
        </div>
        <div id={styles["remember-forgot"]}>
          {/*TODO:*/}
          {/*<label>*/}
          {/*  <Checkbox defaultChecked label="Запомнить пароль" color="gray" />*/}
          {/*</label>*/}
          <Link to={"/reset"}>Забыли пароль?</Link>
        </div>
        <Button id={styles.but_login} type="submit">
          Войти
        </Button>
        <div id={styles.Register_link}>
          <p>
            <Link to={"/register"}>Создать аккаунт</Link>
          </p>
        </div>
      </form>
    </div>
  );
};

export const Route = createFileRoute("/login")({
  component: Page,
  beforeLoad: ({ context, location }) => {
    if (context.auth.isAuthenticated) {
      throw redirect({
        to: "/",
        search: {
          redirect: location.href,
        },
      });
    }
  },
});
