import { invoke } from '@tauri-apps/api/core';
import type { CreateProfileRequest, Profile } from '../api/types/profile';

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
                profilePictureBytes: await profilePicture?.bytes()
            }
        });
    } catch (err: any) {
        console.error(err);
        throw err || 'Something went wrong';
    }
}
