-- Add migration script here

CREATE TABLE IF NOT EXISTS messages
(
    id      INTEGER PRIMARY KEY NOT NULL,
    message TEXT                NOT NULL
);

INSERT INTO messages (id, message)
values (1, 'Hello World!')