-- Your SQL goes here
CREATE TABLE students(
    id INTEGER  PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    usernames_id INTEGER,
    FOREIGN KEY(usernames_id) REFERENCES usernames(id)
)