CREATE TABLE reactions (
	id BIGSERIAL PRIMARY KEY,
  reaction VARCHAR NOT NULL,
  message_id BIGINT NOT NULL,
  channel_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
    CONSTRAINT fk_message
      FOREIGN KEY(message_id) 
	    REFERENCES messages(id)
);