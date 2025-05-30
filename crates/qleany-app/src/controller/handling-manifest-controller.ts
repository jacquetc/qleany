import {invoke} from "@tauri-apps/api/core";


export type LoadManifestDto = {
    manifest_path: string;
}

export type SaveManifestDto = {
    manifest_path: string;
}


export async function loadManifest(dto: LoadManifestDto): Promise<void> {
    return await invoke("load_manifest", {dto});
}

export async function saveManifest(dto: SaveManifestDto): Promise<void> {
    return await invoke("save_manifest", {dto});
}