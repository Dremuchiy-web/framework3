CREATE TABLE IF NOT EXISTS osdr_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    dataset_id VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    file_count INTEGER NOT NULL DEFAULT 0,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_osdr_data_created_at ON osdr_data(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_osdr_data_dataset_id ON osdr_data(dataset_id);

