import type { UserDto } from "../../../-api/dtos/User.dto.ts";

export interface LoginResponseDto extends UserDto {
  token: string;
}
