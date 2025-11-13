export type Profile = {
    id: number;
    username: string;
    display_name?: string;
    avatar?: string;
};

export type CreateProfileRequest = {
    username: string;
    displayName?: string;
    profilePicture?: File;
};
