import autoBind from "auto-bind";
import { queryOptions } from "@tanstack/react-query";
import { appAxios } from "../../../api/config";
import type { LoginResponseDto } from "./dtos/LoginResponse.dto.ts";
import type { LoginDto } from "./dtos/Login.dto.ts";

class _LoginApi {
  readonly baseQueryKey = ["Login"] as const;

  constructor() {
    autoBind(this);
  }

  login(loginDto: LoginDto) {
    return queryOptions({
      queryKey: [...this.baseQueryKey, "login"],
      queryFn: async (): Promise<LoginResponseDto> => {
        const { data } = await appAxios.post<LoginResponseDto>(
          "/login",
          loginDto,
        );
        return data;
      },
    });
  }
}

export const LoginApi = new _LoginApi();
