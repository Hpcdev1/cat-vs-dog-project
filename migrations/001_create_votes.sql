CREATE TABLE IF NOT EXISTS votes (
    voter_id   TEXT PRIMARY KEY,
    choice     TEXT NOT NULL CHECK (choice IN ('cat', 'dog')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);