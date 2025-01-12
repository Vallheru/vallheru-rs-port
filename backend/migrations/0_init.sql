DROP TABLE IF EXISTS player CASCADE;
DROP TABLE IF EXISTS token CASCADE;
DROP TYPE IF EXISTS PLAYER_RANK;

CREATE TYPE PLAYER_RANK AS ENUM ('admin', 'staff', 'member');


CREATE TABLE player (
    id SERIAL PRIMARY KEY,
    username VARCHAR(128) UNIQUE NOT NULL, -- TODO: Think about unique.
    password VARCHAR(72) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    login_count INT NOT NULL DEFAULT 1, -- we update this value after player data is retrieved during logging in
    last_login TIMESTAMPTZ,
    rank PLAYER_RANK NOT NULL DEFAULT 'member',
    last_page VARCHAR(128) NOT NULL DEFAULT '',

    -- game properties
    level INT NOT NULL DEFAULT 1,
    exp INT NOT NULL DEFAULT 0,
    hp INT NOT NULL DEFAULT 15,
    max_hp INT NOT NULL DEFAULT 15,
    sp INT NOT NULL DEFAULT 3,
    energy INT NOT NULL DEFAULT 500,
    max_energy INT NOT NULL DEFAULT 100000,
    inc_energy INT NOT NULL DEFAULT 500,

    gold INT NOT NULL DEFAULT 1000,
    bank INT NOT NULL DEFAULT 0,
    mithrill INT NOT NULL DEFAULT 0,
    vallars INT NOT NULL DEFAULT 0
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

INSERT INTO player (username, password, email, rank) VALUES('admin', '$2b$12$mzgQhjnYGgzAWX5jnI6JaOlbzjgjJ/ppEZpSd.kUtMhwA7LcAou6K', 'admin@vallheru.pl', 'admin'); -- password is admin