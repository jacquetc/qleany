import {invoke} from "@tauri-apps/api/core";


export enum EntityRelationshipField {
    Parent = "Parent",
    Fields = "Fields",
    Relationships = "Relationships"
}

export type CreateEntityDTO = {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    allow_direct_access: boolean;
    fields: number[];
    relationships: number[];
}

export type EntityDto = {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    allow_direct_access: boolean;
    fields: number[];
    relationships: number[];
}

export type EntityRelationshipDto = {
    id: number;
    field: string;
    right_ids: number[];
}


export async function createEntity(dto: CreateEntityDTO): Promise<EntityDto> {
    return await invoke("create_entity", {dto});
}

export async function createEntityMulti(dtos: CreateEntityDTO[]): Promise<EntityDto[]> {
    return await invoke("create_entity_multi", {dtos});
}

export async function getEntity(id: number): Promise<EntityDto | null> {
    return await invoke("get_entity", {id});
}

export async function getEntityMulti(ids: number[]): Promise<(EntityDto | null)[]> {
    return await invoke("get_entity_multi", {ids});
}

export async function updateEntity(dto: EntityDto): Promise<EntityDto> {
    return await invoke("update_entity", {dto});
}

export async function updateEntityMulti(dtos: EntityDto[]): Promise<EntityDto[]> {
    return await invoke("update_entity_multi", {dtos});
}

export async function removeEntity(id: number): Promise<void> {
    return await invoke("remove_entity", {id});
}

export async function removeEntityMulti(ids: number[]): Promise<void> {
    return await invoke("remove_entity_multi", {ids});
}

export async function getEntityRelationship(id: number, field: string): Promise<number[]> {
    return await invoke("get_entity_relationship", {id, field});
}

export async function setEntityRelationship(dto: EntityRelationshipDto): Promise<void> {
    return await invoke("set_entity_relationship", {dto});
}
