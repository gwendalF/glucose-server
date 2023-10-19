-- Add migration script here
BEGIN;

CREATE TABLE IF NOT EXISTS user_account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(127) NOT NULL UNIQUE,
    password VARCHAR(255),
    salt VARCHAR(63),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    CHECK (
        (
            CASE
                WHEN password IS NULL THEN 0
                ELSE 1
            END
        ) + (
            CASE
                WHEN salt IS NULL THEN 0
                ELSE 1
            END
        ) != 1
    )
);


CREATE TABLE IF NOT EXISTS provider_user_mapper(
    id SERIAL PRIMARY KEY,
    name VARCHAR(63) UNIQUE NOT NULL CHECK(name IN ('google', 'facebook')),
    subject TEXT UNIQUE NOT NULL,
    user_id BIGINT NOT NULL REFERENCES user_account(id)
);



CREATE TABLE cgm_values (
    time TIMESTAMPTZ NOT NULL,
    type VARCHAR(32) NOT NULL,
    date BIGINT NOT NULL CHECK (date > 0),
    sgv SMALLINT NOT NULL CHECK (sgv > 0),
    direction VARCHAR(32),
    noise SMALLINT CHECK (ABS(noise) < 45),
    user_id BIGINT NOT NULL REFERENCES user_account(id)
);

SELECT create_hypertable('cgm_values', 'time');

COMMIT;