import type { TemplateWithCategoriesDto } from "./TemplateWithCategories.dto.ts";

export interface CategoryWithTemplatesDto {
  id: number;
  name: string;
  templates: TemplateWithCategoriesDto[];
}
