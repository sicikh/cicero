import { queryOptions } from "@tanstack/react-query";
import autoBind from "auto-bind";
import { appAxios } from "../../../../api/config.ts";
import type { TypeDto } from "./dtos/Type.dto.ts";

class _ConstructorApi {
  readonly baseQueryKey = ["Constructor"] as const;

  constructor() {
    autoBind(this);
  }

  getTemplateDocxById(templateId: number) {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "templates", templateId, "docx"],
      queryFn: async (): Promise<ArrayBuffer> => {
        const { data } = await appAxios.get<ArrayBuffer>(
          `/templates/${templateId}/docx`,
          { responseType: "arraybuffer" },
        );

        return data;
      },
    } as const);
  }

  getTemplateDslById(templateId: number) {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "templates", templateId, "dsl"],
      queryFn: async (): Promise<string> => {
        const { data } = await appAxios.get<string>(
          `/templates/${templateId}/dsl`,
        );

        return data;
      },
    } as const);
  }

  getTemplateDslTypesById(templateId: number) {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "templates", templateId, "dsl", "types"],
      queryFn: async (): Promise<TypeDto[]> => {
        const { data } = await appAxios.get<TypeDto[]>(
          `/templates/${templateId}/dsl/types`,
        );

        return data;
      },
    } as const);
  }
}

export const ConstructorApi = new _ConstructorApi();
