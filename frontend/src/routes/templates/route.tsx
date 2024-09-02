import {Accordion, Divider, Stack, TextInput} from "@mantine/core";
import {IconSearch, IconFolder} from "@tabler/icons-react";
import { useSuspenseQuery } from "@tanstack/react-query";
import { Link, Outlet, createFileRoute } from "@tanstack/react-router";
import React, {useState} from "react";
import { TemplatesApi } from "./-api/templates.api.ts";
import styles from "./route.module.css";

const Page: React.FC = () => {
    const [value, setValue] = useState("")



    const { data: categories } = useSuspenseQuery(
        TemplatesApi.getCategoriesWithTemplates(),
    );

    const filterTemplates = categories.map((category) => {
        const filteredTemplates = category.templates.filter(template =>
            (template.name).toLowerCase().includes(value.toLowerCase())
        );

        return {
            ...category,
            templates: filteredTemplates,
        };
    }).filter(category => category.templates.length > 0);

    return (
        <div className={styles.Container}>
           <Stack
               className={styles.leftSide}
               align="stretch"
               justify="start"
               gap="md"
           >
               <form action="">
                    <TextInput
                        className={styles.search}
                        placeholder="Search"
                        leftSection={<IconSearch/>}
                        onChange={(event) => setValue(event.target.value)}
                    />
               </form>
                <Accordion multiple>
                    {filterTemplates.map((category) => (
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