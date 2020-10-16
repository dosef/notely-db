CREATE TABLE items (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    list_id uuid NOT NULL,
    title VARCHAR NOT NULL
)