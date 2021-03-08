CREATE TABLE messages (
	id BIGSERIAL PRIMARY KEY,
    embed_ids JSONB NOT NULL,
    msg_ids JSONB NOT NULL
);