create table blobs (
  id text primary key,
  data bytea not null,
  name text not null,
  content_type text not null,
  byte_size integer not null,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
