
import {createFileRoute, Link} from "@tanstack/react-router";
import { Title, Stack, Button} from "@mantine/core";
import classes from "./route.module.css"

const Page: React.FC = () => {
    return (
        <Stack
            className={classes.group}
            align="flex-start"
            justify="flex-start"
            gap="lg"
        >
            <Title className={classes.title}>
                Проект <div className={classes.TitleName}>Cicero</div>- это удобный инструмент для создания юридически значимых документов.
            </Title>
            <div className={classes.text}>
                Это быстрый и надёжный способ получить готовый договор или другой юридический
                документ, соответствующий вашим потребностям. Сэкономьте время и упростите
                процесс создания документации  с нашим конструктором документов.
            </div>
            <Link to="/templates"
            >
                <Button className={classes.button}>
                    Создать свой первый документ
                </Button>
            </Link>
        </Stack>
    );
}

export const Route = createFileRoute("/")({
    component: Page,
});
