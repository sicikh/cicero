import { createFileRoute, Link } from "@tanstack/react-router";
import styles from "./route.module.css";
import type React from "react";
import { useRef, useState } from "react";
import {
  Autocomplete,
  Button,
  Checkbox,
  Loader,
  PasswordInput,
} from "@mantine/core";
import {useAuth} from "../../hooks/AuthProvider.tsx";

const Page: React.FC = () => {
  const { login } = useAuth();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const timeoutRef = useRef<number>(-1);
  const [value, setValue] = useState("");
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<string[]>([]);



  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const result = await login({ email, password });
    if (!result.success) {
      setError(result.message || "Login failed");
      console.log(result.message || "Log failed");
    }
    if (result.success) {
      console.log("success");
    }
  };


  const handleChange = (val: string) => {
    window.clearTimeout(timeoutRef.current);
    setValue(val);
    setData([]);

    if (val.trim().length === 0 || val.includes("@")) {
      setLoading(false);
    } else {
      setLoading(true);
      timeoutRef.current = window.setTimeout(() => {
        setLoading(false);
        setData(
          ["mail.ru", "gmail.com", "outlook.com"].map(
            (provider) => `${val}@${provider}`,
          ),
        );
      }, 1000);
    }
  };

  return (
    <div id={styles.wrapper}>
      <Link to="/" className={styles["home-button"]}>
        На главную
      </Link>
      <form action="" onSubmit={handleSubmit} className={styles.form}>
        <h1>Войти</h1>
        <div className={styles["input-box"]}>
          <Autocomplete
            variant="unstyled"
            size="lg"
            data={data}
            value={email}
            onChange={setEmail}

            rightSection={loading ? <Loader size="1rem" /> : null}
            placeholder="E-mail"
          />
        </div>
        <div className={styles["input-box"]}>
          <PasswordInput
              placeholder="Пароль"
              size="lg"
              variant="unstyled"
              value={password}
              onChange={(e) => setPassword(e.currentTarget.value)}
          />
        </div>
        <div id={styles["remember-forgot"]}>
          <label>
            <Checkbox defaultChecked label="Запомнить пароль" color="gray" />
          </label>
          <Link to={"/lostPassword"}>Забыли пароль?</Link>
        </div>
        <Button id={styles.but_login} type="submit">
          Login
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
});
