CREATE TABLE posts (
  id TEXT NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  public BOOLEAN NOT NULL DEFAULT 0,
  draft BOOLEAN NOT NULL DEFAULT 1,
  user_id TEXT NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
)
