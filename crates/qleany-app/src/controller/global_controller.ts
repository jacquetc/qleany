import {invoke} from "@tauri-apps/api/core";


export type CreateGlobalDTO = {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type GlobalDto = {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export async function createGlobal(dto: CreateGlobalDTO): Promise<GlobalDto> {
    return await invoke("create_global", {dto});
}

export async function createGlobalMulti(dtos: CreateGlobalDTO[]): Promise<GlobalDto[]> {
    return await invoke("create_global_multi", {dtos});
}

export async function getGlobal(id: number): Promise<GlobalDto | null> {
    return await invoke("get_global", {id});
}

export async function getGlobalMulti(ids: number[]): Promise<(GlobalDto | null)[]> {
    return await invoke("get_global_multi", {ids});
}

export async function updateGlobal(dto: GlobalDto): Promise<GlobalDto> {
    return await invoke("update_global", {dto});
}

export async function updateGlobalMulti(dtos: GlobalDto[]): Promise<GlobalDto[]> {
    return await invoke("update_global_multi", {dtos});
}

export async function removeGlobal(id: number): Promise<void> {
    return await invoke("remove_global", {id});
}

export async function removeGlobalMulti(ids: number[]): Promise<void> {
    return await invoke("remove_global_multi", {ids});
}
