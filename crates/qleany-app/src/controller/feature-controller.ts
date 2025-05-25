import {invoke} from "@tauri-apps/api/core";

export enum FeatureRelationshipField {
    UseCases = "UseCases",
}

export type CreateFeatureDTO = {
    name: string;
    use_cases: number[];
}

export type FeatureDto = {
    id: number;
    name: string;
    use_cases: number[];
}

export type FeatureRelationshipDto = {
    id: number;
    field: FeatureRelationshipField;
    right_ids: number[];
}

export async function createFeature(dto: CreateFeatureDTO): Promise<FeatureDto> {
    return await invoke("create_feature", {dto});
}

export async function createFeatureMulti(dtos: CreateFeatureDTO[]): Promise<FeatureDto[]> {
    return await invoke("create_feature_multi", {dtos});
}

export async function getFeature(id: number): Promise<FeatureDto | null> {
    return await invoke("get_feature", {id});
}

export async function getFeatureMulti(ids: number[]): Promise<(FeatureDto | null)[]> {
    return await invoke("get_feature_multi", {ids});
}

export async function updateFeature(dto: FeatureDto): Promise<FeatureDto> {
    return await invoke("update_feature", {dto});
}

export async function updateFeatureMulti(dtos: FeatureDto[]): Promise<FeatureDto[]> {
    return await invoke("update_feature_multi", {dtos});
}

export async function removeFeature(id: number): Promise<void> {
    return await invoke("remove_feature", {id});
}

export async function removeFeatureMulti(ids: number[]): Promise<void> {
    return await invoke("remove_feature_multi", {ids});
}

export async function getFeatureRelationship(id: number, field: FeatureRelationshipField): Promise<number[]> {
    return await invoke("get_feature_relationship", {id, field});
}

export async function setFeatureRelationship(dto: FeatureRelationshipDto): Promise<void> {
    return await invoke("set_feature_relationship", {dto});
}
