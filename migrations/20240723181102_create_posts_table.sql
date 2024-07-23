CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title varchar(64) NOT NULL,
    body varchar(4096)
);
