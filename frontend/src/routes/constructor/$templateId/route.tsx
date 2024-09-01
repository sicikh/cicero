import {
  Button,
  Divider,
  NumberInput,
  Radio,
  TextInput,
  Container,
  Group,
  TypographyStylesProvider, ScrollArea, ActionIcon
} from "@mantine/core";
import { type FormApi, type ReactFormApi, useForm } from "@tanstack/react-form";
import {createFileRoute, Link, useLoaderData} from "@tanstack/react-router";
import type React from "react";
import { useEffect } from "react";
import { useMemo, useState } from "react";
import { ConstructorApi } from "./-api/constructor.api.ts";
import type { TypeDto } from "./-api/dtos/Type.dto.ts";
import styles from "./route.module.css";
import datastyles from "./cssInputs/data-input.module.css";
import stringstyles from "./cssInputs/string-input.module.css";
import numberstyles from "./cssInputs/number-input.module.css";
import radiostyles from "./cssInputs/radio-input.module.css";
import "dayjs/locale/ru";
import { DateInput, type DateInputProps } from "@mantine/dates";
import * as dayjs from "dayjs";
import * as DocxPreview from "docx-preview";
import Docxtemplater from "docxtemplater";
import expressionParser from "docxtemplater/expressions";
import FileSaver from "file-saver";
import PizZip from "pizzip";
import { IconArrowNarrowLeft } from '@tabler/icons-react';

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
  const { templateId } = Route.useParams();


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

  const [visibleContainer, setVisibleContainer] = useState('first');

  return (
    <div>
      <Container fluid h={61} className={styles.container}>
        <Group className={styles.headerGroup}>
          <Link
              to={"/templates/$templateId"}
              params={{ templateId }}
          >
              <ActionIcon size="lg" variant="outline" color="rgba(222, 222, 222, 1)" aria-label="Back">
                <IconArrowNarrowLeft stroke={1.5} />
              </ActionIcon>
          </Link>

          <Group className={styles.previewGroup}>
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
              <Button className={styles.Button1} variant="gradient" gradient={{ from: '#C8420D', to: '#842500', deg: 90 }} onClick={() => setVisibleContainer((prevState) => prevState === 'first' ? 'second' : 'first')}
              >
                {visibleContainer === "first" ? 'Show Form' : 'Show Docx'}
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
              <div className={styles.inputsForm}>
                <FormField
                    key={typeDto.name}
                    form={form}
                    typeDto={typeDto}
                    level={1}
                    parent={undefined}
                    isEnumField={false}
                />
              </div>
          ))}
          </ScrollArea>
        </form>
        <Divider className="h-full" orientation="vertical"/>
        <ScrollArea className={styles.scrollbar1} type="scroll" scrollbars="y" offsetScrollbars scrollHideDelay={1500}>
          <div id={"docx-container"}/>
        </ScrollArea>
      </div>
      {visibleContainer === "first" ? (
          <ScrollArea className={styles.scrollbar1} type="scroll" scrollbars="y" offsetScrollbars scrollHideDelay={1500}>
            <div id={"docx-container"}/>
          </ScrollArea>
      ) : (
          <form
              id="constructor"
              className={styles.inputsForm}
              onSubmit={(e) => {
                e.preventDefault();
                form.handleSubmit();
              }}
          >
            <ScrollArea className={styles.scrollbar} type="scroll" scrollbars="y" offsetScrollbars
                        scrollHideDelay={1500}>
              {dslTypes.map((typeDto) => (
                  <div className={styles.inputsForm}>
                    <FormField
                        key={typeDto.name}
                        form={form}
                        typeDto={typeDto}
                        level={1}
                        parent={undefined}
                        isEnumField={false}
                    />
                  </div>
              ))}
            </ScrollArea>
          </form>
      )}
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
                <>
                  <div className={stringstyles.StringInput}>
                    <div className={stringstyles.StringLabel}>{!isEnumField ? replaceTags(typeDto.comment) : ""}</div>
                    <TextInput
                        className={stringstyles.stringInput}
                        size="md"
                        variant="unstyled"
                        placeholder={replaceTags(typeDto.comment)}
                        withAsterisk={typeDto.isRequired}
                        value={field.state.value === undefined ? "" : field.state.value}
                        onChange={(e) => field.handleChange(e.target.value)}
                        onBlur={field.handleBlur}
                    />
                  </div>

                </>


            );
          }

          if (typeDto.type === "Integer") {
            return (
                <>
                  <div className={numberstyles.NumberInput}>
                    <div className={numberstyles.numberLabel}>{!isEnumField ? replaceTags(typeDto.comment) : ""}</div>
                      <NumberInput
                          className={numberstyles.numberInput}
                          placeholder={replaceTags(typeDto.comment)}
                          size="md"
                          variant="unstyled"
                          withAsterisk={typeDto.isRequired}
                          value={field.state.value as number}
                          onChange={(value) => field.handleChange(value)}
                          onBlur={field.handleBlur}
                      />
                  </div>

                </>

            );
          }

          if (typeDto.type === "Date") {
            const dateParser: DateInputProps["dateParser"] = (input) => {
              return dayjs(input, "DD.MM.YYYY").toDate();
            };

            return (
              <>
                <div className={datastyles.DataInput}>
                  <div className={datastyles.dateLabel}>{!isEnumField ? replaceTags(typeDto.comment) : ""}</div>
                  <DateInput
                      className={datastyles.dateInput}
                      locale="ru"
                      variant="unstyled"
                      size="md"
                      clearable
                      dateParser={dateParser}
                      valueFormat="DD.MM.YYYY"
                      placeholder={replaceTags(typeDto.comment)}
                      withAsterisk={typeDto.isRequired}
                      value={field.state.value}
                      onChange={(value) => {
                        field.handleChange(value as Date);
                      }}
                      onBlur={field.handleBlur}
                  />
                </div>

              </>
            );
          }

          if (typeDto.type === "Struct") {
            return (
              <>
                <TypographyStylesProvider>
                  {/* biome-ignore lint/security/noDangerouslySetInnerHtml: <explanation> */}
                  <div className={styles.groupInputTitle} dangerouslySetInnerHTML={{ __html: typeDto.comment }} />
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
                <>
                  <div className={radiostyles.RadioInput}>
                    <div className={radiostyles.RadioLabel}>{replaceTags(typeDto.comment)}</div>
                    <Radio.Group
                        className={radiostyles.radioInput}
                        withAsterisk={typeDto.isRequired}
                        value={field.state.value as string}
                        // error={field.state.meta.touchedErrors.at(0)}
                        onChange={(value) => {
                          setSelectedRadio(value);
                          field.handleChange(value);
                        }}
                        onBlur={field.handleBlur}
                    >
                      <Group mt="xs" className={radiostyles.radioPos}>
                        {typeDto.variants.map((it) => (
                            <div  className={styles.exFormInput} key={it.name}>
                              <Radio
                                  label={replaceTags(it.comment)}
                                  value={it.name}
                                  iconColor="#343A40"
                                  color='#C8420D'
                                  styles={{
                                    label: {
                                      color: '#868686',
                                      fontSize: 16,
                                    }
                                  }}
                              />
                            </div>

                        ))}
                      </Group>
                    </Radio.Group>

                  </div>
                  {typeDto.variants.map((it) => (
                      <div  className={styles.exFormInput} key={it.name}>
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
                </>

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
  loader: async ({params: {templateId}, context: {queryClient}}) => {
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

    return {docx, dsl, dslTypes};
  },
  component: Page,
});
