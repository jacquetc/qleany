import {invoke} from "@tauri-apps/api/core";


export type CreateRelationshipDTO = {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type RelationshipDto = {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type RelationshipRelationshipDto = {
    id: number;
    field: string;
    right_ids: number[];
}

export async function createRelationship(dto: CreateRelationshipDTO): Promise<RelationshipDto> {
    return await invoke("create_relationship", {dto});
}

export async function createRelationshipMulti(dtos: CreateRelationshipDTO[]): Promise<RelationshipDto[]> {
    return await invoke("create_relationship_multi", {dtos});
}

export async function getRelationship(id: number): Promise<RelationshipDto | null> {
    return await invoke("get_relationship", {id});
}

export async function getRelationshipMulti(ids: number[]): Promise<(RelationshipDto | null)[]> {
    return await invoke("get_relationship_multi", {ids});
}

export async function updateRelationship(dto: RelationshipDto): Promise<RelationshipDto> {
    return await invoke("update_relationship", {dto});
}

export async function updateRelationshipMulti(dtos: RelationshipDto[]): Promise<RelationshipDto[]> {
    return await invoke("update_relationship_multi", {dtos});
}

export async function removeRelationship(id: number): Promise<void> {
    return await invoke("remove_relationship", {id});
}

export async function removeRelationshipMulti(ids: number[]): Promise<void> {
    return await invoke("remove_relationship_multi", {ids});
}

export async function getRelationshipRelationship(id: number, field: string): Promise<number[]> {
    return await invoke("get_relationship_relationship", {id, field});
}

export async function setRelationshipRelationship(dto: RelationshipRelationshipDto): Promise<void> {
    return await invoke("set_relationship_relationship", {dto});
}
