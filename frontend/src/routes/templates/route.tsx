import { Accordion, Button, Divider, Title } from "@mantine/core";
import { IconFolder } from "@tabler/icons-react";
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
      <div className={styles.Categories}>
        <Title order={5}>Выберите шаблон из представленных категорий</Title>
        <Accordion multiple>
          {categories.map((category) => (
            <Accordion.Item key={category.id} value={category.name}>
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
                    <Button key={template.id}>{template.name}</Button>
                  </Link>
                ))}
              </Accordion.Panel>
            </Accordion.Item>
          ))}
        </Accordion>
      </div>
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
