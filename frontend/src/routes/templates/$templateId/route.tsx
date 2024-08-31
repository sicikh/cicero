import {Button, Stack, Text, Title} from "@mantine/core";
import { useSuspenseQuery } from "@tanstack/react-query";
import { Link, createFileRoute } from "@tanstack/react-router";
import type React from "react";
import { TemplatesApi } from "../-api/templates.api.ts";
import styles from "./route.module.css";

const Page: React.FC = () => {
  const { templateId } = Route.useParams();
  const { data: template } = useSuspenseQuery(
    TemplatesApi.getTemplateById(Number.parseInt(templateId, 10)),
  );

  return (
      <div className={styles.rightSide}>
          <div className={styles.mainRS}>
              <Stack
                  align="stretch"
                  justify="start"
                  gap="xs"
              >
                  <Title>
                      {template.name}
                  </Title>
                  <Text>
                      {template.description}
                  </Text>
              </Stack>
              <Link to={"/constructor/$templateId"} params={{ templateId: templateId }}>
                  <Button
                      className={styles.button}
                      fullWidth
                      variant="outline"
                      size="lg"
                      color="#495057"
                      radius="xl"
                  >
                      Создать договор
                  </Button>
              </Link>
          </div>
      </div>
  );
};

export const Route = createFileRoute("/templates/$templateId")({
  loader: async ({ params: { templateId }, context: { queryClient } }) =>
    queryClient.ensureQueryData(
      TemplatesApi.getTemplateById(Number.parseInt(templateId, 10)),
    ),
  component: Page,
});
