CREATE TABLE IF NOT EXISTS astronomy_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    object_name VARCHAR(255) NOT NULL,
    object_type VARCHAR(100) NOT NULL,
    ra DOUBLE PRECISION NOT NULL,
    dec DOUBLE PRECISION NOT NULL,
    magnitude DOUBLE PRECISION,
    distance_ly DOUBLE PRECISION,
    observation_date TIMESTAMPTZ NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(object_name, observation_date)
);

CREATE INDEX IF NOT EXISTS idx_astronomy_data_observation_date ON astronomy_data(observation_date DESC);
CREATE INDEX IF NOT EXISTS idx_astronomy_data_object_name ON astronomy_data(object_name);

