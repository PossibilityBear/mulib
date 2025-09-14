
CREATE TABLE IF NOT EXISTS PlaylistsSongs
(
    [playlist_id]   INTEGER     NOT NULL,
    [song_id]       INTEGER     NOT NULL,
    [track_number]  INTEGER     NOT NULL,
    PRIMARY KEY (playlist_id, song_id),
    FOREIGN KEY ([playlist_id]) REFERENCES Playlists([id]),
    FOREIGN KEY ([song_id]) REFERENCES Songs([id])
)