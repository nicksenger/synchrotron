CREATE TABLE documents (
  id SERIAL PRIMARY KEY,
  title VARCHAR (255) UNIQUE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE pages (
  id SERIAL PRIMARY KEY,
  page_number INT NOT NULL,
  image_path TEXT NOT NULL,
  aspect_ratio REAL NOT NULL,
  height REAL NOT NULL,
  document INT NOT NULL REFERENCES documents(id) ON DELETE CASCADE
);

CREATE TABLE bookmarks (
  id SERIAL PRIMARY KEY,
  title VARCHAR (255) NOT NULL,
  document_page INT NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
  document INT NOT NULL REFERENCES documents(id) ON DELETE CASCADE
);

CREATE TABLE tracks (
  id SERIAL PRIMARY KEY,
  track_number INT NOT NULL,
  title VARCHAR (255) NOT NULL,
  audio_path TEXT NOT NULL,
  document INT NOT NULL REFERENCES documents(id) ON DELETE CASCADE
);

CREATE TABLE anchors (
  id SERIAL PRIMARY KEY,
  title TEXT,
  track_time REAL NOT NULL, 
  position_top REAL NOT NULL,
  position_left REAL NOT NULL,
  document_page INT NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
  track INT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE user_anchors (
  id SERIAL PRIMARY KEY,
  title TEXT,
  track_time REAL NOT NULL, 
  position_top REAL NOT NULL,
  position_left REAL NOT NULL,
  document_page INT NOT NULL REFERENCES pages(id) ON DELETE CASCADE,
  track INT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
  owning_user INT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);
