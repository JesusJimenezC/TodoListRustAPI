-- Your SQL goes here
CREATE TABLE tasks
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    date        TEXT             DEFAULT (strftime('%Y-%m-%d', 'now')),
    list_id     INTEGER NOT NULL,
    FOREIGN KEY (list_id) REFERENCES lists (id)
);