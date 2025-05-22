import {invoke} from "@tauri-apps/api/core";


export type CreateUseCaseDTO = {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type UseCaseDto = {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type UseCaseRelationshipDto = {
    id: number;
    field: string;
    right_ids: number[];
}

export async function createUseCase(dto: CreateUseCaseDTO): Promise<UseCaseDto> {
    return await invoke("create_use_case", {dto});
}

export async function createUseCaseMulti(dtos: CreateUseCaseDTO[]): Promise<UseCaseDto[]> {
    return await invoke("create_use_case_multi", {dtos});
}

export async function getUseCase(id: number): Promise<UseCaseDto | null> {
    return await invoke("get_use_case", {id});
}

export async function getUseCaseMulti(ids: number[]): Promise<(UseCaseDto | null)[]> {
    return await invoke("get_use_case_multi", {ids});
}

export async function updateUseCase(dto: UseCaseDto): Promise<UseCaseDto> {
    return await invoke("update_use_case", {dto});
}

export async function updateUseCaseMulti(dtos: UseCaseDto[]): Promise<UseCaseDto[]> {
    return await invoke("update_use_case_multi", {dtos});
}

export async function removeUseCase(id: number): Promise<void> {
    return await invoke("remove_use_case", {id});
}

export async function removeUseCaseMulti(ids: number[]): Promise<void> {
    return await invoke("remove_use_case_multi", {ids});
}

export async function getUseCaseRelationship(id: number, field: string): Promise<number[]> {
    return await invoke("get_use_case_relationship", {id, field});
}

export async function setUseCaseRelationship(dto: UseCaseRelationshipDto): Promise<void> {
    return await invoke("set_use_case_relationship", {dto});
}
