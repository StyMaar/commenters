DROP TABLE comments;

CREATE TABLE comments (
  id INTEGER NOT NULL PRIMARY KEY,
  article VARCHAR NOT NULL,
  message VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  date VARCHAR NOT NULL,
  uuid VARCHAR NOT NULL
);
