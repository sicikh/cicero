import { queryOptions } from "@tanstack/react-query";
import autoBind from "auto-bind";
import { appAxios } from "../../../api/config.ts";
import type { CategoryWithTemplatesDto } from "./dtos/CategoryWithTemplates.dto.ts";
import type { TemplateCategoryDto } from "./dtos/TemplateCategory.dto.ts";
import type { TemplateWithCategoriesDto } from "./dtos/TemplateWithCategories.dto.ts";

class _TemplatesApi {
  readonly baseQueryKey = ["Templates"] as const;

  constructor() {
    autoBind(this);
  }

  getCategoriesWithTemplates() {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "templates"],
      queryFn: async (): Promise<CategoryWithTemplatesDto[]> => {
        const { data } =
          await appAxios.get<TemplateWithCategoriesDto[]>("/templates");

        const categoriesMap = new Map<number, CategoryWithTemplatesDto>();

        for (const template of data) {
          for (const category of template.categories) {
            if (!categoriesMap.has(category.id)) {
              categoriesMap.set(category.id, {
                id: category.id,
                name: category.name,
                templates: [],
              });
            }
          }
        }

        for (const template of data) {
          for (const category of template.categories) {
            // biome-ignore lint/style/noNonNullAssertion: already checked in the previous loop
            categoriesMap.get(category.id)!.templates.push(template);
          }
        }

        let categories = Array.from(categoriesMap.values());
        categories = categories.sort((a, b) => b.id - a.id);

        return categories;
      },
    } as const);
  }

  getCategories() {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "categories"],
      queryFn: async (): Promise<TemplateCategoryDto[]> => {
        const { data } =
          await appAxios.get<TemplateWithCategoriesDto[]>("/templates");

        const categoriesMap = new Map<number, TemplateCategoryDto>();

        for (const template of data) {
          for (const category of template.categories) {
            if (!categoriesMap.has(category.id)) {
              categoriesMap.set(category.id, {
                id: category.id,
                name: category.name,
              });
            }
          }
        }

        let categories = Array.from(categoriesMap.values());
        categories = categories.sort((a, b) => b.id - a.id);

        return categories;
      },
    } as const);
  }

  getTemplateById(templateId: number) {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "templates", templateId],
      queryFn: async (): Promise<TemplateWithCategoriesDto> => {
        const { data } = await appAxios.get<TemplateWithCategoriesDto>(
          `/templates/${templateId}`,
        );

        return data;
      },
    } as const);
  }
}

export const TemplatesApi = new _TemplatesApi();
