import type { UserDto } from "../../../-api/dtos/User.dto.ts";

export type PublicityDto =
  | {
      type: "public";
    }
  | {
      type: "private";
      viewers: UserDto[];
    };
