import {
  Button,
  Divider,
  NumberInput,
  Radio,
  TextInput,
  Container,
  Group,
  TypographyStylesProvider, ScrollArea,
} from "@mantine/core";
import { type FormApi, type ReactFormApi, useForm } from "@tanstack/react-form";
import { createFileRoute, useLoaderData } from "@tanstack/react-router";
import type React from "react";
import { useEffect } from "react";
import { useMemo, useState } from "react";
import { ConstructorApi } from "./-api/constructor.api.ts";
import type { TypeDto } from "./-api/dtos/Type.dto.ts";
import styles from "./route.module.css";
import "dayjs/locale/ru";
import { DateInput, type DateInputProps } from "@mantine/dates";
import * as dayjs from "dayjs";
import * as DocxPreview from "docx-preview";
import Docxtemplater from "docxtemplater";
import expressionParser from "docxtemplater/expressions";
import FileSaver from "file-saver";
import PizZip from "pizzip";

type PrimitiveValue = string | number | boolean | Date;

// @ts-ignore
interface EnumValue {
  _discriminant: string;
  _discriminantField?: PrimitiveValue | FormValues;
}
// @ts-ignore
type FormValues = Record<string, PrimitiveValue | FormValues | EnumValue>;

const Page: React.FC = () => {
  const { docx, dslTypes } = useLoaderData({
    from: "/constructor/$templateId",
  });
  const [templateContext, setTemplateContext] = useState<FormValues>({});
  const [renderedDocxFile, setRenderedDocxFile] = useState<Blob | undefined>();

  useEffect(() => {
    try {
      const zip = new PizZip(docx);
      const doc = new Docxtemplater(zip, {
        paragraphLoop: true,
        linebreaks: true,
        parser: expressionParser,
        nullGetter: (part) => {
          if (!part.module) {
            return "____";
          }
          if (part.module === "rawxml") {
            return "";
          }
          return "";
        },
      });

      doc.render(templateContext);

      const renderedDocx = doc.getZip().generate({ type: "arraybuffer" });
      const renderedDocxFile = doc.getZip().generate({
        type: "blob",
        mimeType:
          "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      });
      setRenderedDocxFile(() => renderedDocxFile);
      DocxPreview.renderAsync(
        renderedDocx,
        // biome-ignore lint/style/noNonNullAssertion: we are sure that element exists
        document.querySelector("#docx-container")!,
      );
    } catch (error) {
      console.error("Error processing DOCX:", error);
    }
  }, [templateContext, docx]);

  const formData = useMemo(() => {
    const defaultValues: FormValues = {};

    const setDefaultValue = (
      typeDto: TypeDto,
      defaultValue: FormValues,
      isEnumField: boolean,
    ) => {
      const name = isEnumField ? "_discriminantField" : typeDto.name;

      if (typeDto.type === "String") {
        defaultValue[name] = "";
      }
      if (typeDto.type === "Struct") {
        defaultValue[name] = {};

        for (const field of typeDto.fields) {
          setDefaultValue(field, defaultValue[name], false);
        }
      }
      if (typeDto.type === "Enum") {
        const variant = typeDto.variants[0];
        if ("isRequired" in variant) {
          defaultValue[typeDto.name] = { _discriminant: variant.name };
          setDefaultValue(variant, defaultValue[name], true);
        } else {
          defaultValue[typeDto.name] = { _discriminant: variant.name };
        }
      }
    };

    for (const typeDto of dslTypes) {
      setDefaultValue(typeDto, defaultValues, false);
    }

    return {
      defaultValues: defaultValues,
    };
  }, [dslTypes]);

  const form = useForm<FormValues>({
    defaultValues: formData.defaultValues,
    onSubmit: async ({ value }) => {
      setTemplateContext(() => structuredClone(value));

      console.log(value);
    },
  });

  return (
    <div>
      <Container fluid h={61} className={styles.container}>
        <Group className={styles.headerGroup}>
          <Group className={styles.previewGroup}>
            <div className={styles.previewText}>Предварительный просмотр документа</div>
            <Group className={styles.buttonGroup}>
              {renderedDocxFile !== undefined && (
                  <Button
                      className={styles.Button}
                      color="#DEE2E6"
                      onClick={() => FileSaver.saveAs(renderedDocxFile, "template.docx")}
                  >
                    Сохранить шаблон
                  </Button>
              )}
              <Button type="submit" className={styles.Button} color="#DEE2E6" form="constructor">
                Обновить
              </Button>
            </Group>
          </Group>
        </Group>
      </Container>
      <div className={styles.Container}>
        <form
            id="constructor"
            className={styles.inputsForm}
            onSubmit={(e) => {
              e.preventDefault();
              form.handleSubmit();
            }}
        >
          <ScrollArea className={styles.scrollbar} type="scroll" scrollbars="y" offsetScrollbars scrollHideDelay={1500}>
          {dslTypes.map((typeDto) => (
              <FormField
                  key={typeDto.name}
                  form={form}
                  typeDto={typeDto}
                  level={1}
                  parent={undefined}
                  isEnumField={false}
              />
          ))}
          </ScrollArea>
        </form>
        <Divider className={"h-full"} orientation={"vertical"}/>
        <ScrollArea className={styles.scrollbar} type="scroll" scrollbars="y" offsetScrollbars scrollHideDelay={1500}>
          <div id={"docx-container"} className={styles.overview}/>
        </ScrollArea>
      </div>

    </div>
  );
};

interface FormFieldProps {
  form: FormApi<FormValues> & ReactFormApi<FormValues>;
  typeDto: TypeDto;
  level: number;
  parent: string | undefined;
  isEnumField: boolean;
}

const FormField: React.FC<FormFieldProps> = ({
                                               form,
                                               typeDto,
                                               level,
                                               parent,
                                               isEnumField,
                                             }) => {
  let name: string;
  if (!isEnumField) {
    name = parent ? `${parent}.${typeDto.name}` : typeDto.name;
  } else {
    // biome-ignore lint/style/noNonNullAssertion: we are a field, so we have a parent
    name = parent!;
  }

  const oldName = name;
  if (isEnumField) {
    name = `${name}._discriminantField`;
  } else if (typeDto.type === "Enum") {
    name = `${name}._discriminant`;
  }
  const [selectedRadio, setSelectedRadio] = useState<string | undefined>();

  return (
    <div className={styles.Group}>
      <form.Field
        name={name}
        validators={{
          onChange: typeDto.isRequired
            ? ({ value }) =>
                value === undefined || value === ""
                  ? "Поле обязательно для заполнения"
                  : undefined
            : undefined,
        }}
      >
        {(field) => {
          if (typeDto.type === "String") {
            return (
              <TextInput
                label={!isEnumField ? replaceTags(typeDto.comment) : ""}
                placeholder={replaceTags(typeDto.comment)}
                withAsterisk={typeDto.isRequired}
                error={field.state.meta.errors.at(0)}
                value={field.state.value === undefined ? "" : field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
                onBlur={field.handleBlur}
              />
            );
          }

          if (typeDto.type === "Integer") {
            return (
              <NumberInput
                label={!isEnumField ? replaceTags(typeDto.comment) : ""}
                placeholder={replaceTags(typeDto.comment)}
                withAsterisk={typeDto.isRequired}
                error={field.state.meta.errors.at(0)}
                value={field.state.value as number}
                onChange={(value) => field.handleChange(value)}
                onBlur={field.handleBlur}
              />
            );
          }

          if (typeDto.type === "Date") {
            const dateParser: DateInputProps["dateParser"] = (input) => {
              return dayjs(input, "DD.MM.YYYY").toDate();
            };

            return (
              <>
                <DateInput
                  locale="ru"
                  clearable
                  dateParser={dateParser}
                  valueFormat="DD.MM.YYYY"
                  error={field.state.meta.errors.at(0)}
                  label={!isEnumField ? replaceTags(typeDto.comment) : ""}
                  placeholder={replaceTags(typeDto.comment)}
                  withAsterisk={typeDto.isRequired}
                  value={field.state.value}
                  onChange={(value) => {
                    field.handleChange(value as Date);
                  }}
                  onBlur={field.handleBlur}
                />
              </>
            );
          }

          if (typeDto.type === "Struct") {
            return (
              <>
                <TypographyStylesProvider>
                  {/* biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation> */}
                  <div dangerouslySetInnerHTML={{ __html: typeDto.comment }} />
                </TypographyStylesProvider>
                {typeDto.fields.map((field) => (
                  <FormField
                    key={field.name}
                    form={form}
                    typeDto={field}
                    level={level + 1}
                    parent={name}
                    isEnumField={false}
                  />
                ))}
              </>
            );
          }

          if (typeDto.type === "Enum") {
            return (
              <Radio.Group
                label={replaceTags(typeDto.comment)}
                withAsterisk={typeDto.isRequired}
                value={field.state.value as string}
                // error={field.state.meta.touchedErrors.at(0)}
                onChange={(value) => {
                  setSelectedRadio(value);
                  field.handleChange(value);
                }}
                onBlur={field.handleBlur}
              >
                <div className="flex flex-col gap-1">
                  {typeDto.variants.map((it) => (
                    <div key={it.name}>
                      <Radio label={replaceTags(it.comment)} value={it.name} />
                      {selectedRadio === it.name && "isRequired" in it ? (
                        <FormField
                          form={form}
                          typeDto={it}
                          level={level + 1}
                          parent={oldName}
                          isEnumField
                        />
                      ) : undefined}
                    </div>
                  ))}
                </div>
              </Radio.Group>
            );
          }
        }}
      </form.Field>
    </div>
  );
};

const replaceTags = (comment: string) => {
  const withoutTags = comment.replace(/<[^>]*>/g, "");
  return withoutTags.replace(":", "");
};

export const Route = createFileRoute("/constructor/$templateId")({
  loader: async ({ params: { templateId }, context: { queryClient } }) => {
    const [docx, dsl, dslTypes] = await Promise.all([
      queryClient.ensureQueryData(
        ConstructorApi.getTemplateDocxById(Number.parseInt(templateId, 10)),
      ),
      queryClient.ensureQueryData(
        ConstructorApi.getTemplateDslById(Number.parseInt(templateId, 10)),
      ),
      queryClient.ensureQueryData(
        ConstructorApi.getTemplateDslTypesById(Number.parseInt(templateId, 10)),
      ),
    ]);

    return { docx, dsl, dslTypes };
  },
  component: Page,
});
