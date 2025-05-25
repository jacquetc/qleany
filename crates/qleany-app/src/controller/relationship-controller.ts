import {invoke} from "@tauri-apps/api/core";

export enum RelationshipType {
    OneToOne = "OneToOne",
    OneToMany = "OneToMany",
    ManyToOne = "ManyToOne",
    ManyToMany = "ManyToMany"
}

export enum Strength {
    Weak = "Weak",
    Strong = "Strong"
}

export enum Direction {
    Forward = "Forward",
    Backward = "Backward"
}

export enum Cardinality {
    ZeroOrOne = "ZeroOrOne",
    One = "One",
    ZeroOrMore = "ZeroOrMore",
    OneOrMore = "OneOrMore"
}

export enum Order {
    Ordered = "Ordered",
    Unordered = "Unordered"
}

export enum RelationshipRelationshipField {
    LeftEntity = "LeftEntity",
    RightEntity = "RightEntity"
}

export type CreateRelationshipDto = {
    left_entity: number;
    right_entity: number;
    field_name: string;
    relationship_type: RelationshipType;
    strength: Strength;
    direction: Direction;
    cardinality: Cardinality;
    order: Order | null;
}

export type RelationshipDto = {
    id: number;
    left_entity: number;
    right_entity: number;
    field_name: string;
    relationship_type: RelationshipType;
    strength: Strength;
    direction: Direction;
    cardinality: Cardinality;
    order: Order | null;
}

export type RelationshipRelationshipDto = {
    id: number;
    field: RelationshipRelationshipField;
    right_ids: number[];
}

export async function createRelationship(dto: CreateRelationshipDto): Promise<RelationshipDto> {
    return await invoke("create_relationship", {dto});
}

export async function createRelationshipMulti(dtos: CreateRelationshipDto[]): Promise<RelationshipDto[]> {
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

export async function getRelationshipRelationship(id: number, field: RelationshipRelationshipField): Promise<number[]> {
    return await invoke("get_relationship_relationship", {id, field});
}

export async function setRelationshipRelationship(dto: RelationshipRelationshipDto): Promise<void> {
    return await invoke("set_relationship_relationship", {dto});
}
