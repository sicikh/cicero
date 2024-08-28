import { Button, Text, Title } from "@mantine/core";
import { Link, createFileRoute } from "@tanstack/react-router";
import type React from "react";
import styles from "./route.module.css";

const Page: React.FC = () => (
  <div className={styles.Container}>
    <div className={styles.TextBlock}>
      <Title order={3}>
        Проект «Cicero» — это удобный инструмент для создания юридически
        значимых документов
      </Title>
      <Text>
        Это быстрый и надежный способ получить готовый договор или другой
        юридический документ, соответствующий вашим потребностям. Сэкономьте
        время и упростите процесс создания документации с нашим конструктором
        документов.
      </Text>
    </div>
    <Link to={"/templates"}>
      <Button className="px-20">Создать свой первый документ</Button>
    </Link>
  </div>
);

export const Route = createFileRoute("/")({
  component: Page,
});
