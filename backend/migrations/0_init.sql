DROP TABLE player;

CREATE TABLE player (
    id SERIAL PRIMARY KEY,
    username VARCHAR(60) UNIQUE NOT NULL, -- TODO: Think about unique.
    password VARCHAR(72) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

CREATE TABLE token (
    player_id INT NOT NULL,
    token VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ NOT NULL DEFAULT NOW() + '12 hours'::INTERVAL,
    PRIMARY KEY(player_id, token),
    CONSTRAINT fk_player
        FOREIGN KEY(player_id)
            REFERENCES player(id)
);

INSERT INTO player (username, password, email) VALUES('admin', '$2b$12$mzgQhjnYGgzAWX5jnI6JaOlbzjgjJ/ppEZpSd.kUtMhwA7LcAou6K', 'admin@vallheru.pl'); -- password is admin