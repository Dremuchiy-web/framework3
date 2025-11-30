CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS iss_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    altitude DOUBLE PRECISION NOT NULL,
    velocity DOUBLE PRECISION NOT NULL,
    visibility VARCHAR(50) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL UNIQUE,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_iss_data_timestamp ON iss_data(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_iss_data_fetched_at ON iss_data(fetched_at DESC);

