
CREATE TABLE IF NOT EXISTS Albums
(
    [id]          INTEGER     NOT NULL,
    [title]       TEXT        NOT NULL,
    [artist_id]   INTEGER,
    PRIMARY KEY ([id]),
    FOREIGN KEY ([artist_id]) REFERENCES Artists([id])
)