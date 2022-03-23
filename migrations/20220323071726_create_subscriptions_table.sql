-- Add migration script here
CREATE TABLE subscriptions(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT not NULL,
  name TEXT NOT NULL,
  subscribed_at timestamptz not NULL
);
