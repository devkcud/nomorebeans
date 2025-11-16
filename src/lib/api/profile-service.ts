import { invoke } from '@tauri-apps/api/core';
import type { CreateProfileRequest, Profile, UpdateProfileRequest } from '../api/types/profile';

export async function getProfiles(): Promise<Profile[]> {
    const profiles = await invoke<Profile[]>('get_profiles');

    return profiles.map((profile) => ({
        ...profile,
        avatar: profile.avatar ? `data:image/webp;base64,${profile.avatar}` : undefined
    }));
}

export async function createProfile(profile: CreateProfileRequest): Promise<Profile> {
    try {
        const { username, displayName, profilePicture } = profile;

        return await invoke<Profile>('create_profile', {
            profile: {
                username,
                displayName: displayName?.trim() === '' ? undefined : displayName,
                // @ts-ignore
                profilePictureBytes: await profilePicture?.bytes()
            }
        });
    } catch (err: any) {
        console.error(err);
        throw err || 'Something went wrong';
    }
}

export async function updateProfile(id: number, profile: UpdateProfileRequest): Promise<Profile> {
    try {
        const { username, displayName, profilePicture } = profile;

        const result = await invoke<Profile>('update_profile', {
            id,
            profile: {
                username: username?.trim() === '' ? undefined : username,
                displayName: displayName?.trim() === '' ? undefined : displayName,
                // @ts-ignore
                profilePictureBytes: await profilePicture?.bytes()
            }
        });

        return {
            ...result,
            avatar: result.avatar ? `data:image/webp;base64,${result.avatar}` : undefined
        };
    } catch (err: any) {
        console.error(err);
        throw err || 'Something went wrong';
    }
}

export async function deleteProfile(id: number): Promise<void> {
    try {
        await invoke('delete_profile', { id });
    } catch (err: any) {
        console.error(err);
        throw err || 'Something went wrong';
    }
}
