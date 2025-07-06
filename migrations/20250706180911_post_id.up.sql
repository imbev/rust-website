-- Add up migration script here

ALTER TABLE posts
ADD id uuid PRIMARY KEY DEFAULT gen_random_uuid()
;
