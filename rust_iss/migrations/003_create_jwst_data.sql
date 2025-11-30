CREATE TABLE IF NOT EXISTS jwst_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    observation_id VARCHAR(255) NOT NULL UNIQUE,
    target_name VARCHAR(255) NOT NULL,
    instrument VARCHAR(100) NOT NULL,
    observation_type VARCHAR(100) NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_jwst_data_start_time ON jwst_data(start_time DESC);
CREATE INDEX IF NOT EXISTS idx_jwst_data_observation_id ON jwst_data(observation_id);

