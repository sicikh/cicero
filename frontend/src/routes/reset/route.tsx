import { Autocomplete, Button, Loader } from "@mantine/core";
import { Link, createFileRoute } from "@tanstack/react-router";
import type React from "react";
import { useRef, useState } from "react";
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
      <Link to={"/login"} className={styles["home-button"]}>
        Назад
      </Link>
      <form action="" className={styles.form}>
        <h1>Забыли пароль</h1>
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
        <Button id={styles.but_login} type="submit">
          Выслать письмо
        </Button>
      </form>
    </div>
  );
};

export const Route = createFileRoute("/reset")({
  component: Page,
});
