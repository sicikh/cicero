export type TypeDto = PrimitiveTypeDto | StructTypeDto | EnumTypeDto;

interface BaseDto {
  name: string;
  comment: string;
  isRequired: boolean;
}

export interface PrimitiveTypeDto extends BaseDto {
  type: "String" | "Integer" | "Date";
}

export interface StructTypeDto extends BaseDto {
  type: "Struct";
  typeName: string;
  typeNameComment?: string;
  fields: TypeDto[];
}

export interface EnumTypeDto extends BaseDto {
  type: "Enum";
  typeName: string;
  typeNameComment?: string;
  variants: (EnumVariantDto | TypeDto)[];
}

export interface EnumVariantDto {
  name: string;
  comment: string;
}

// TODO: elementType is strained TypeDto, so it does not contain name, comment, isRequired
// export interface ArrayTypeDto extends BaseDto {
//     type: "Array";
//     elementType: TypeDto;
// }
