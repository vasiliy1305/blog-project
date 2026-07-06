CREATE TABLE IF NOT EXISTS posts(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT,
    author_id BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT  now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT  now(),

    FOREIGN KEY (author_id) 
    REFERENCES  users(id) 
    ON DELETE CASCADE
);

CREATE INDEX  IF NOT EXISTS idx_posts_created_at
ON posts(created_at);

CREATE INDEX  IF NOT EXISTS idx_posts_author_id
ON posts(author_id);