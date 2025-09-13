CREATE TABLE IF NOT EXISTS Songs
(
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    file_path TEXT NOT NULL,
    album_id INTEGER,
    artist_id INTEGER
)