import {invoke} from "@tauri-apps/api/core";

export enum DtoFieldType {
    Boolean = "Boolean",
    Integer = "Integer",
    UInteger = "UInteger",
    Float = "Float",
    String = "String",
    Uuid = "Uuid",
    DateTime = "DateTime",
}

export type CreateDtoFieldDTO = {
    name: string;
    field_type: DtoFieldType;
    is_nullable: boolean;
    is_list: boolean;
}

export type DtoFieldDto = {
    id: number;
    name: string;
    field_type: DtoFieldType;
    is_nullable: boolean;
    is_list: boolean;
}

export async function createDtoField(dto: CreateDtoFieldDTO): Promise<DtoFieldDto> {
    return await invoke("create_dto_field", {dto});
}

export async function createDtoFieldMulti(dtos: CreateDtoFieldDTO[]): Promise<DtoFieldDto[]> {
    return await invoke("create_dto_field_multi", {dtos});
}

export async function getDtoField(id: number): Promise<DtoFieldDto | null> {
    return await invoke("get_dto_field", {id});
}

export async function getDtoFieldMulti(ids: number[]): Promise<(DtoFieldDto | null)[]> {
    return await invoke("get_dto_field_multi", {ids});
}

export async function updateDtoField(dto: DtoFieldDto): Promise<DtoFieldDto> {
    return await invoke("update_dto_field", {dto});
}

export async function updateDtoFieldMulti(dtos: DtoFieldDto[]): Promise<DtoFieldDto[]> {
    return await invoke("update_dto_field_multi", {dtos});
}

export async function removeDtoField(id: number): Promise<void> {
    return await invoke("remove_dto_field", {id});
}

export async function removeDtoFieldMulti(ids: number[]): Promise<void> {
    return await invoke("remove_dto_field_multi", {ids});
}

