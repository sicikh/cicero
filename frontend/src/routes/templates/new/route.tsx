import { Button, Container, Group, Radio, TextInput } from "@mantine/core";
import { useForm } from "@tanstack/react-form";
import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import type React from "react";
import type { TemplateCategoryDto } from "../-api/dtos/TemplateCategory.dto.ts";
// import { useSuspenseQuery } from "@tanstack/react-query";
// import { TemplatesApi } from "../-api/templates.api.ts";
import { appAxios } from "../../../api/config.ts";

interface FormValues {
  name: string;
  description: string;
  categories: TemplateCategoryDto[];
  publicity: string;
  viewers: string[];
  docxFile: File | null;
  dslFile: File | null;
}

const Page: React.FC = () => {
  // const categories = useSuspenseQuery(TemplatesApi.getCategories());
  const navigate = useNavigate({ from: "/templates/new" });

  const form = useForm<FormValues>({
    defaultValues: {
      name: "",
      description: "",
      categories: [],
      publicity: "public",
      viewers: [],
      docxFile: null,
      dslFile: null,
    },
    onSubmit: async ({ value }) => {
      const data = {
        name: value.name,
        description: value.description,
        categories: [1],
        publicity: value.publicity,
        viewers: value.viewers,
      };

      const formData = new FormData();
      formData.append("json", JSON.stringify(data));
      // biome-ignore lint/style/noNonNullAssertion: <explanation>
      formData.append("docx", value.docxFile!);
      // biome-ignore lint/style/noNonNullAssertion: <explanation>
      formData.append("dsl", value.dslFile!);

      const res = await appAxios.post("/templates", formData);

      if (res.status !== 200) {
        alert("Ошибка при создании шаблона");
      } else {
        navigate({ to: "/templates" });
      }
    },
  });

  return (
    <Container>
      <form
        id={"templates-new"}
        onSubmit={(e) => {
          e.preventDefault();
          form.handleSubmit();
        }}
      >
        <form.Field
          name={"name"}
          validators={{
            onChange: ({ value }) => {
              if (value === "") {
                return "Поле обязательно для заполнения";
              }
            },
          }}
        >
          {(field) => {
            return (
              <TextInput
                placeholder={"Название шаблона"}
                required
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
                error={field.state.meta.errors.at(0)}
              />
            );
          }}
        </form.Field>
        <form.Field
          name={"description"}
          validators={{
            onChange: ({ value }) => {
              if (value === "") {
                return "Поле обязательно для заполнения";
              }
            },
          }}
        >
          {(field) => {
            return (
              <TextInput
                placeholder={"Описание шаблона"}
                required
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
                error={field.state.meta.errors.at(0)}
              />
            );
          }}
        </form.Field>
        <form.Field name={"publicity"}>
          {(field) => {
            return (
              <Radio.Group
                withAsterisk={true}
                value={field.state.value as string}
                onChange={(value) => {
                  field.handleChange(value);
                }}
                onBlur={field.handleBlur}
              >
                <Group>
                  <Radio label={"Публичный"} value={"public"} />
                  <Radio label={"Приватный"} value={"private"} />
                </Group>
              </Radio.Group>
            );
          }}
        </form.Field>
        <form.Field
          name={"docxFile"}
          validators={{
            onChange: ({ value }) => {
              if (!value) {
                return "Поле обязательно для заполнения";
              }
            },
          }}
        >
          {(field) => {
            return (
              <input
                type={"file"}
                accept={".docx"}
                required
                onChange={(e) => {
                  field.handleChange(e.target.files?.[0] ?? null);
                }}
              />
            );
          }}
        </form.Field>
        <form.Field
          name={"dslFile"}
          validators={{
            onChange: ({ value }) => {
              if (!value) {
                return "Поле обязательно для заполнения";
              }
            },
          }}
        >
          {(field) => {
            return (
              <input
                type={"file"}
                accept={".dsl"}
                required
                onChange={(e) => {
                  field.handleChange(e.target.files?.[0] ?? null);
                }}
              />
            );
          }}
        </form.Field>
        <Button type={"submit"}>Создать шаблон</Button>
      </form>
    </Container>
  );
};

export const Route = createFileRoute("/templates/new")({
  component: Page,
  beforeLoad: ({ context }) => {
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: "/login",
        search: {
          redirect: "/templates/new",
        },
      });
    }
  },
});
