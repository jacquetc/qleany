import {invoke} from "@tauri-apps/api/core";

export enum DtoRelationshipField {
    Fields = "Fields",
}

export type CreateDtoDTO = {
    name: string;
    fields: number[];
}

export type DtoDto = {
    id: number;
    name: string;
    fields: number[];
}

export type DtoRelationshipDto = {
    id: number;
    field: DtoRelationshipField;
    right_ids: number[];
}

export async function createDto(dto: CreateDtoDTO): Promise<DtoDto> {
    return await invoke("create_dto", {dto});
}

export async function createDtoMulti(dtos: CreateDtoDTO[]): Promise<DtoDto[]> {
    return await invoke("create_dto_multi", {dtos});
}

export async function getDto(id: number): Promise<DtoDto | null> {
    return await invoke("get_dto", {id});
}

export async function getDtoMulti(ids: number[]): Promise<(DtoDto | null)[]> {
    return await invoke("get_dto_multi", {ids});
}

export async function updateDto(dto: DtoDto): Promise<DtoDto> {
    return await invoke("update_dto", {dto});
}

export async function updateDtoMulti(dtos: DtoDto[]): Promise<DtoDto[]> {
    return await invoke("update_dto_multi", {dtos});
}

export async function removeDto(id: number): Promise<void> {
    return await invoke("remove_dto", {id});
}

export async function removeDtoMulti(ids: number[]): Promise<void> {
    return await invoke("remove_dto_multi", {ids});
}

export async function getDtoRelationship(id: number, field: DtoRelationshipField): Promise<number[]> {
    return await invoke("get_dto_relationship", {id, field});
}

export async function setDtoRelationship(dto: DtoRelationshipDto): Promise<void> {
    return await invoke("set_dto_relationship", {dto});
}
