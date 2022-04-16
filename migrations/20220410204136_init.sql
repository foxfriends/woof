CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(32) UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content TEXT NOT NULL,
    author UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post UUID NOT NULL REFERENCES posts (id) ON DELETE CASCADE
);

CREATE TABLE votes (
    voter UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post UUID NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    positive BOOLEAN NOT NULL,
    PRIMARY KEY (post, voter)
);
