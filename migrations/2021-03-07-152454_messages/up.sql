CREATE TABLE messages (
	id BIGINT PRIMARY KEY,
    others JSONB NOT NULL,
    reactions JSONB NOT NULL
);