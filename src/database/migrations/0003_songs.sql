CREATE TABLE IF NOT EXISTS Songs
(
    [id]          INTEGER     NOT NULL,
    [title]       TEXT        NOT NULL,
    [file_path]   TEXT        NOT NULL,
    [album_id]    INTEGER,
    [artist_id]   INTEGER,
    PRIMARY KEY ([id]),
    FOREIGN KEY ([album_id]) REFERENCES Albums([id]),
    FOREIGN KEY ([artist_id]) REFERENCES Artists([id])
)