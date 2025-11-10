import { invoke } from '@tauri-apps/api/core';
import type { Profile } from '../api/types/profile';

export async function getProfiles(): Promise<Profile[]> {
    return await invoke<Profile[]>('get_profiles');
}
