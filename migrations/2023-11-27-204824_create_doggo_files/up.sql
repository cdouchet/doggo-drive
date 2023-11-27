CREATE TABLE IF NOT EXISTS doggo_files (
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    local_path TEXT NOT NULL,
    net_path TEXT NOT NULL,
    file_name TEXT
);