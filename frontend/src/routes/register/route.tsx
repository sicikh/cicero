import { Autocomplete, Loader, PasswordInput, TextInput } from "@mantine/core";
import { Link, createFileRoute } from "@tanstack/react-router";
import { useRef, useState } from "react";
import type React from "react";
import styles from "./route.module.css";

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
        <Link to="/" className="text-[#EEEEEE] hover:text-[#ffffff]">
          <i className="bx bx-left-arrow-alt" />
        </Link>
        <h1>Регистрация</h1>
        <div className={styles["input-box"]}>
          <TextInput id="input-box" type="text" placeholder="Имя" required />
        </div>
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
          <PasswordInput
            className={styles.inputBox}
            variant="unstyled"
            placeholder="Пароль"
            size="lg"
          />
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
});
