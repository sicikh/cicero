import {Accordion, Divider, Stack, TextInput} from "@mantine/core";
import {IconSearch, IconFolder} from "@tabler/icons-react";
import { useSuspenseQuery } from "@tanstack/react-query";
import { Link, Outlet, createFileRoute } from "@tanstack/react-router";
import type React from "react";
import { TemplatesApi } from "./-api/templates.api.ts";
import styles from "./route.module.css";

const Page: React.FC = () => {
    const { data: categories } = useSuspenseQuery(
        TemplatesApi.getCategoriesWithTemplates(),
    );

    return (
        <div className={styles.Container}>
           <Stack
               className={styles.leftSide}
               align="stretch"
               justify="start"
               gap="md"
           >
                <TextInput
                    className={styles.search}
                    placeholder="Search"
                    leftSection={<IconSearch/>}
                />
                <Accordion multiple>
                    {categories.map((category) => (
                        <Accordion.Item className={styles.accordion} key={category.id} value={category.name}>
                            <Accordion.Control icon={<IconFolder />}>
                                {category.name}
                            </Accordion.Control>
                            <Accordion.Panel>
                                {category.templates.map((template) => (
                                    <Link
                                        key={template.id}
                                        to={"/templates/$templateId"}
                                        params={{ templateId: template.id.toString() }}
                                    >
                                        {template.name}
                                    </Link>
                                ))}
                            </Accordion.Panel>
                        </Accordion.Item>
                    ))}
                </Accordion>
           </Stack>
            <Divider className="h-full" orientation="vertical" />
            <Outlet />
        </div>
    );
};

export const Route = createFileRoute("/templates")({
    loader: async ({ context: { queryClient } }) =>
        queryClient.ensureQueryData(TemplatesApi.getCategoriesWithTemplates()),
    component: Page,
});