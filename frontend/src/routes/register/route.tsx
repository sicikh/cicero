import { Autocomplete, Loader, PasswordInput, TextInput } from "@mantine/core";
import { useForm } from "@tanstack/react-form";
import { Link, createFileRoute, redirect } from "@tanstack/react-router";
import { useRef, useState } from "react";
import type React from "react";
import { appAxios } from "../../api/config.ts";
import type { RegisterDto } from "./-api/dtos/Register.dto.ts";
import styles from "./route.module.css";

const Page: React.FC = () => {
  const timeoutRef = useRef<number>(-1);
  const [isLoading, setIsLoading] = useState(false);
  const [emailProviders, setEmailProviders] = useState<string[]>([]);

  const form = useForm<RegisterDto>({
    defaultValues: {
      name: "",
      email: "",
      password: "",
    },
    onSubmit: async ({ value }) => {
      const res = await appAxios.post("/auth/register", value);

      if (res.status !== 200) {
        alert("Произошла ошибка при регистрации");
        return;
      }

      alert(
        "На указанную почту выслано письмо для подтверждения. " +
          "Пройдите по ссылке в письме, чтобы подтвердить регистрацию.",
      );
    },
  });

  return (
    <div id={styles.wrapper}>
      <Link to="/" className={styles["home-button"]}>
        На главную
      </Link>

      <form
        id="register"
        className={styles.form}
        onSubmit={(e) => {
          e.preventDefault();
          form.handleSubmit();
        }}
      >
        <Link to="/" className="text-[#EEEEEE] hover:text-[#ffffff]">
          <i className="bx bx-left-arrow-alt" />
        </Link>
        <h1>Регистрация</h1>
        <div className={styles["input-box"]}>
          <form.Field
            name={"name"}
            validators={{
              onChange: ({ value }) =>
                value === "" ? "Поле обязательно для заполнения" : undefined,
            }}
          >
            {(field) => {
              return (
                <TextInput
                  id="input-box"
                  type="text"
                  placeholder="Имя"
                  required
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)}
                  error={field.state.meta.errors.at(0)}
                />
              );
            }}
          </form.Field>
        </div>
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
        {/*<div id={styles['remember-forgot']} className="md:flex justify-end">*/}
        {/*    <a href="#" className="hover:underline">Забыли пароль?</a>*/}
        {/*</div>*/}
        <button id={styles.but_login} type="submit">
          Создать аккаунт
        </button>
        <div id={styles.Register_link}>
          <p>
            Есть аккаунт?
            <Link
              to={"/login"}
              className="font-semibold hover:underline ml-[5px]"
            >
              Войти
            </Link>
          </p>
        </div>
      </form>
    </div>
  );
};
export const Route = createFileRoute("/register")({
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
