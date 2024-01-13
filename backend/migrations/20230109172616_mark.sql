CREATE TABLE IF NOT EXISTS mark (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    time TEXT NOT NULL,
    images TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL
);
