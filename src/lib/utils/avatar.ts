import { AVATAR_PLACEHOLDER_API } from '$lib/constants';

export function getPlaceholderAvatarUrl(username: string): string {
    return `${AVATAR_PLACEHOLDER_API}?seed=${encodeURIComponent(username)}`;
}

export function getAvatarUrl(avatar: string | undefined, username: string): string {
    return avatar ?? getPlaceholderAvatarUrl(username);
}

export function createManagedObjectUrl(file: File): { url: string; revoke: () => void } {
    const url = URL.createObjectURL(file);

    return {
        url,
        revoke: () => URL.revokeObjectURL(url)
    };
}
