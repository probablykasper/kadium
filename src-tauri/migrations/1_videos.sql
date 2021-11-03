CREATE TABLE IF NOT EXISTS videos
(
    id                TEXT    NOT NULL PRIMARY KEY,
    title             TEXT    NOT NULL,
    thumbnailStandard BOOLEAN NOT NULL,
    thumbnailMaxres   BOOLEAN NOT NULL,
    description       TEXT    NOT NULL,
    channelId         TEXT    NOT NULL,
    unread            BOOLEAN NOT NULL
);
