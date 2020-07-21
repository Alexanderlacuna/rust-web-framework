-- Your SQL goes here

CREATE TABLE  person(
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);
CREATE TABLE  gender(
    id INTEGER  PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    person_id INTEGER,
    FOREIGN KEY(person_id) REFERENCES person(id)
)