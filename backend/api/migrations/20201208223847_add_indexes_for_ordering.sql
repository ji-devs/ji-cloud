-- Add migration script here
ALTER TABLE subject
    ADD COLUMN index int2 check("index" >= 0) UNIQUE;

ALTER TABLE style
    ADD COLUMN index int2 check("index" >= 0) UNIQUE;

ALTER TABLE age_range
    ADD COLUMN index int2 check("index" >= 0) UNIQUE;

ALTER TABLE affiliation
    ADD COLUMN index int2 check("index" >= 0) UNIQUE;

ALTER TABLE content_type
    ADD COLUMN index int2 check("index" >= 0) UNIQUE;