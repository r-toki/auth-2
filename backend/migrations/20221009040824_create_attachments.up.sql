create table attachments (
  id text primary key,
  record_type text,
  record_id text,
  record_name text,
  blob_id text,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
