import {invoke} from "@tauri-apps/api/core";

export type CreateFileDTO = {
    name: string;
    group: string;
}

export type FileDto = {
    id: number;
    name: string;
    group: string;
}

export async function createFile(dto: CreateFileDTO): Promise<FileDto> {
    return await invoke("create_file", {dto});
}

export async function createFileMulti(dtos: CreateFileDTO[]): Promise<FileDto[]> {
    return await invoke("create_file_multi", {dtos});
}

export async function getFile(id: number): Promise<FileDto | null> {
    return await invoke("get_file", {id});
}

export async function getFileMulti(ids: number[]): Promise<(FileDto | null)[]> {
    return await invoke("get_file_multi", {ids});
}

export async function updateFile(dto: FileDto): Promise<FileDto> {
    return await invoke("update_file", {dto});
}

export async function updateFileMulti(dtos: FileDto[]): Promise<FileDto[]> {
    return await invoke("update_file_multi", {dtos});
}

export async function removeFile(id: number): Promise<void> {
    return await invoke("remove_file", {id});
}

export async function removeFileMulti(ids: number[]): Promise<void> {
    return await invoke("remove_file_multi", {ids});
}

