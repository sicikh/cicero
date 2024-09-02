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

const Page: React.FC = () => {
  const timeoutRef = useRef<number>(-1);
  const [value, setValue] = useState("");
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState<string[]>([]);

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
      <form action="" className={styles.form}>
        <h1>Войти</h1>
        <div className={styles["input-box"]}>
          <Autocomplete
            variant="unstyled"
            size="lg"
            value={value}
            data={data}
            onChange={handleChange}
            rightSection={loading ? <Loader size="1rem" /> : null}
            placeholder="E-mail"
          />
        </div>
        <div className={styles["input-box"]}>
          <PasswordInput placeholder="Пароль" size="lg" variant="unstyled" />
        </div>
        <div id={styles["remember-forgot"]}>
          <label>
            <Checkbox defaultChecked label="Запомнить пароль" color="gray" />
          </label>
          {/*TODO: add forgot route*/}
          {/*<Link to={"/login"}>Забыли пароль?</Link>*/}
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
