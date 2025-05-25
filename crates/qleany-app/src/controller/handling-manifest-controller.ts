import {invoke} from "@tauri-apps/api/core";


export type LoadManifestDto = {
    manifest_path: string;
}

export async function loadManifest(dto: LoadManifestDto): Promise<void> {
    return await invoke("load_manifest", {dto});
}