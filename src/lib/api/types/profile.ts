export type Profile = {
    id: number;
    username: string;
    displayName?: string;
    avatar?: string;
};

export type CreateProfileRequest = {
    username: string;
    displayName?: string;
    profilePicture?: File;
};
