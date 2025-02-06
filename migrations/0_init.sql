DROP TABLE IF EXISTS player CASCADE;
DROP TABLE IF EXISTS token CASCADE;
DROP TYPE IF EXISTS PLAYER_RANK;
DROP TYPE IF EXISTS PLAYER_GENDER;
DROP TYPE IF EXISTS PLAYER_RELIGION;
DROP TYPE IF EXISTS PLAYER_RACE;
DROP TYPE IF EXISTS PLAYER_CLASS;

CREATE TYPE PLAYER_RANK AS ENUM ('admin', 'staff', 'member');
CREATE TYPE PLAYER_GENDER AS ENUM('', 'male', 'female');
CREATE TYPE PLAYER_RELIGION AS ENUM(
    '',
    'illuminati',
    'karserth',
    'anariel',
    'heluvald',
    'tartus',
    'oregarl',
    'daeraell',
    'teathe-di',
    'thindil'
);
CREATE TYPE PLAYER_RACE AS ENUM(
    '',
    'human',
    'elf',
    'dwarf',
    'hobbit',
    'reptilian',
    'gnome'
);
CREATE TYPE PLAYER_CLASS AS ENUM(
    '',
    'warrior',
    'mage',
    'craftsman',
    'barbarian',
    'thief'
);

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
    ip INET DEFAULT NULL,
    gender PLAYER_GENDER NOT NULL DEFAULT '',
    protection BOOLEAN NOT NULL DEFAULT true,

    -- game properties
    level INT NOT NULL DEFAULT 1,
    exp INT NOT NULL DEFAULT 0,
    hp INT NOT NULL DEFAULT 15,
    max_hp INT NOT NULL DEFAULT 15,
    sp INT NOT NULL DEFAULT 3,
    energy INT NOT NULL DEFAULT 500,
    max_energy INT NOT NULL DEFAULT 100000,
    inc_energy INT NOT NULL DEFAULT 500,

    last_killed INT DEFAULT NULL,
    last_killed_by INT DEFAULT NULL,
    fights_won INT NOT NULL DEFAULT 0,
    fights_lost INT NOT NULL DEFAULT 0,

    gold INT NOT NULL DEFAULT 1000,
    bank INT NOT NULL DEFAULT 0,
    mithrill INT NOT NULL DEFAULT 0,
    vallars INT NOT NULL DEFAULT 0,

    ap INT NOT NULL DEFAULT 5,
    race PLAYER_RACE NOT NULL DEFAULT '',
    profession PLAYER_CLASS NOT NULL DEFAULT '',
    religion PLAYER_RELIGION NOT NULL DEFAULT '',
    agility NUMERIC(6, 4) NOT NULL DEFAULT 0.0000,
    strength NUMERIC(6, 4) NOT NULL DEFAULT 0.0000,
    intelligence NUMERIC(6, 4) NOT NULL DEFAULT 0.0000,
    wisdom NUMERIC(6, 4) NOT NULL DEFAULT 0.0000,
    speed NUMERIC(6, 4) NOT NULL DEFAULT 0.0000,
    stamina NUMERIC(6, 4) NOT NULL DEFAULT 0.0000
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