import {invoke} from "@tauri-apps/api/core";


export type ListRustFilesDto = {
    only_existing: boolean;
}

export type GenerateRustFilesDto = {
    file_ids: number[];
    root_path: string;
    prefix: string;
}

export type GenerateRustFilesResultDto = {
    files: string[];
    timestamp: string;
}


export async function listRustFiles(dto: ListRustFilesDto): Promise<void> {
    return await invoke("list_rust_files", {dto});
}

export async function generateRustFiles(dto: GenerateRustFilesDto): Promise<string> {
    return await invoke("generate_rust_files", {dto});
}

export async function getGenerateRustFilesResult(operationId: string): Promise<GenerateRustFilesResultDto | null> {
    return await invoke("get_generate_rust_files_result", {operationId});
}
