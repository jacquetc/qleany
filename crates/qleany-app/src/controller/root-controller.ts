import {invoke} from "@tauri-apps/api/core";

export enum RootRelationshipField {
    Global = "Global",
    Entities = "Entities",
    Features = "Features",
    Files = "Files",
}

export type CreateRootDTO = {
    global: number;
    entities: number[];
    features: number[];
    files: number[];
}

export type RootDto = {
    id: number;
    global: number;
    entities: number[];
    features: number[];
    files: number[];
}

export type RootRelationshipDto = {
    id: number;
    field: RootRelationshipField;
    right_ids: number[];
}

export async function createRoot(dto: CreateRootDTO): Promise<RootDto> {
    return await invoke("create_root", {dto});
}

export async function createRootMulti(dtos: CreateRootDTO[]): Promise<RootDto[]> {
    return await invoke("create_root_multi", {dtos});
}

export async function getRoot(id: number): Promise<RootDto | null> {
    return await invoke("get_root", {id});
}

export async function getRootMulti(ids: number[]): Promise<(RootDto | null)[]> {
    return await invoke("get_root_multi", {ids});
}

export async function updateRoot(dto: RootDto): Promise<RootDto> {
    return await invoke("update_root", {dto});
}

export async function updateRootMulti(dtos: RootDto[]): Promise<RootDto[]> {
    return await invoke("update_root_multi", {dtos});
}

export async function removeRoot(id: number): Promise<void> {
    return await invoke("remove_root", {id});
}

export async function removeRootMulti(ids: number[]): Promise<void> {
    return await invoke("remove_root_multi", {ids});
}

export async function getRootRelationship(id: number, field: RootRelationshipField): Promise<number[]> {
    return await invoke("get_root_relationship", {id, field});
}

export async function setRootRelationship(dto: RootRelationshipDto): Promise<void> {
    return await invoke("set_root_relationship", {dto});
}
