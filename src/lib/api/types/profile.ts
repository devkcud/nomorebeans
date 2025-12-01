export type GetProfileResponse = {
    id: number;
    createdAt: Date;
    updatedAt: Date;
    username: string;
    displayName: string;
    avatar: string;
};

export type CreateProfileRequest = {
    username: string;
    displayName?: string;
    profilePicture?: File;
};

export type UpdateProfileRequest = {
    username?: string;
    displayName?: string;
    profilePicture?: File;
};
