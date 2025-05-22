import {invoke} from "@tauri-apps/api/core";

export enum FieldType {
    Boolean = "Boolean",
    Integer = "Integer",
    UInteger = "UInteger",
    Float = "Float",
    String = "String",
    Uuid = "Uuid",
    DateTime = "DateTime",
    Entity = "Entity"
}

export enum FieldRelationshipField {
    Entity = "Entity"
}

export type CreateFieldDTO = {
    name: string;
    field_type: FieldType;
    entity: number | null;
    is_nullable: boolean;
    is_primary_key: boolean;
    is_list: boolean;
    single: boolean;
    strong: boolean;
    ordered: boolean;
    list_model: boolean;
    list_model_displayed_field: string | null;
}

export type FieldDto = {
    id: number;
    name: string;
    field_type: FieldType;
    entity: number | null;
    is_nullable: boolean;
    is_primary_key: boolean;
    is_list: boolean;
    single: boolean;
    strong: boolean;
    ordered: boolean;
    list_model: boolean;
    list_model_displayed_field: string | null;
}

export type FieldRelationshipDto = {
    id: number;
    field: FieldRelationshipField;
    right_ids: number[];
}

export async function createField(dto: CreateFieldDTO): Promise<FieldDto> {
    return await invoke("create_field", {dto});
}

export async function createFieldMulti(dtos: CreateFieldDTO[]): Promise<FieldDto[]> {
    return await invoke("create_field_multi", {dtos});
}

export async function getField(id: number): Promise<FieldDto | null> {
    return await invoke("get_field", {id});
}

export async function getFieldMulti(ids: number[]): Promise<(FieldDto | null)[]> {
    return await invoke("get_field_multi", {ids});
}

export async function updateField(dto: FieldDto): Promise<FieldDto> {
    return await invoke("update_field", {dto});
}

export async function updateFieldMulti(dtos: FieldDto[]): Promise<FieldDto[]> {
    return await invoke("update_field_multi", {dtos});
}

export async function removeField(id: number): Promise<void> {
    return await invoke("remove_field", {id});
}

export async function removeFieldMulti(ids: number[]): Promise<void> {
    return await invoke("remove_field_multi", {ids});
}

export async function getFieldRelationship(id: number, field: FieldRelationshipField): Promise<number[]> {
    return await invoke("get_field_relationship", {id, field});
}

export async function setFieldRelationship(dto: FieldRelationshipDto): Promise<void> {
    return await invoke("set_field_relationship", {dto});
}
