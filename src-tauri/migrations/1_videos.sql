CREATE TABLE IF NOT EXISTS videos
(
    id                TEXT    NOT NULL PRIMARY KEY,
    title             TEXT    NOT NULL,
    description       TEXT    NOT NULL,
    publishTimeMs     INTEGER NOT NULL,
    durationMs        INTEGER NOT NULL,
    thumbnailStandard BOOLEAN NOT NULL,
    thumbnailMaxres   BOOLEAN NOT NULL,
    channelName       TEXT    NOT NULL,
    channelId         TEXT    NOT NULL,
    unread            BOOLEAN NOT NULL DEFAULT 1,
    archived          BOOLEAN NOT NULL DEFAULT 0
);
