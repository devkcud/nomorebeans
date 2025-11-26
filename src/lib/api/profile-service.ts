import { invoke } from '@tauri-apps/api/core';
import type {
    CreateProfileRequest,
    GetProfileResponse,
    UpdateProfileRequest
} from '../api/types/profile';
import { AVATAR_DATA_URI_PREFIX } from '$lib/constants';
import { fileToBytes, getPlaceholderAvatarUrl } from '$lib/utils';

function transformProfile(profile: GetProfileResponse): GetProfileResponse {
    return {
        ...profile,
        avatar: profile.avatar
            ? `${AVATAR_DATA_URI_PREFIX}${profile.avatar}`
            : getPlaceholderAvatarUrl(profile.username),
        displayName:
            profile.displayName?.trim() === ''
                ? profile.username
                : (profile.displayName ?? profile.username)
    };
}

export async function getProfiles(): Promise<GetProfileResponse[]> {
    const profiles = await invoke<GetProfileResponse[]>('get_profiles');
    let p = profiles.map(transformProfile);
    console.log('Transformed profiles:', p);
    return p;
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

    return transformProfile(result);
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

    return transformProfile(result);
}

export async function deleteProfile(id: number): Promise<void> {
    await invoke('delete_profile', { id });
}
