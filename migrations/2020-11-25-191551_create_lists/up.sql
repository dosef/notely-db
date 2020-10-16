CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE lists (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
  title VARCHAR NOT NULL
)