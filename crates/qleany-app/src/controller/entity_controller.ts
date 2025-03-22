import { invoke } from "@tauri-apps/api/core";


export type CreateEntityDTO = {
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export type EntityDto = {
    id: number;
    name: string;
    only_for_heritage: boolean;
    parent: number | null;
    fields: number[];
    relationships: number[];
}

export async function createEntity(dto: CreateEntityDTO): Promise<EntityDto> {
    var res: EntityDto = await invoke("create_entity", { dto });
    return res;
}

