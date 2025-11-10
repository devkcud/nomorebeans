CREATE TABLE IF NOT EXISTS profiles (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT,
    profile_picture_url TEXT
);