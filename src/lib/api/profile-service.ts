import { invoke } from '@tauri-apps/api/core';
import type {
    CreateProfileRequest,
    GetProfileResponse,
    UpdateProfileRequest
} from '../api/types/profile';
import { AVATAR_DATA_URI_PREFIX } from '$lib/constants';
import { fileToBytes } from '$lib/utils';

function transformProfileAvatar(profile: GetProfileResponse): GetProfileResponse {
    return {
        ...profile,
        avatar: profile.avatar ? `${AVATAR_DATA_URI_PREFIX}${profile.avatar}` : undefined
    };
}

export async function getProfiles(): Promise<GetProfileResponse[]> {
    const profiles = await invoke<GetProfileResponse[]>('get_profiles');
    return profiles.map(transformProfileAvatar);
}

export async function createProfile(profile: CreateProfileRequest): Promise<GetProfileResponse> {
    const { username, displayName, profilePicture } = profile;

    const result = await invoke<GetProfileResponse>('create_profile', {
        profile: {
            username,
            displayName: displayName?.trim() === '' ? undefined : displayName,
            profilePictureBytes: profilePicture ? await fileToBytes(profilePicture) : undefined
        }
    });

    return transformProfileAvatar(result);
}

export async function updateProfile(
    id: number,
    profile: UpdateProfileRequest
): Promise<GetProfileResponse> {
    const { username, displayName, profilePicture } = profile;

    const result = await invoke<GetProfileResponse>('update_profile', {
        id,
        profile: {
            username: username?.trim() === '' ? undefined : username,
            displayName: displayName?.trim() === '' ? undefined : displayName,
            profilePictureBytes: profilePicture ? await fileToBytes(profilePicture) : undefined
        }
    });

    return transformProfileAvatar(result);
}

export async function deleteProfile(id: number): Promise<void> {
    await invoke('delete_profile', { id });
}
