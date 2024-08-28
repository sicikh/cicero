import type { UserDto } from "../../../-api/dtos/User.dto.ts";
import type { PublicityDto } from "./Publicity.dto.ts";
import type { TemplateCategoryDto } from "./TemplateCategory.dto.ts";

export interface TemplateWithCategoriesDto {
  id: number;
  name: string;
  description: string;
  author: UserDto;
  publicity: PublicityDto;
  categories: TemplateCategoryDto[];
}
