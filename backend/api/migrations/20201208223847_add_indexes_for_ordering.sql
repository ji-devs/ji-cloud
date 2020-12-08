-- Add migration script here
ALTER TABLE subject
    ADD COLUMN index int2 NOT NULL check("index" >= 0) UNIQUE;

ALTER TABLE style
    ADD COLUMN index int2 NOT NULL check("index" >= 0) UNIQUE;

ALTER TABLE age_range
    ADD COLUMN index int2 NOT NULL check("index" >= 0) UNIQUE;

ALTER TABLE affiliation
    ADD COLUMN index int2 NOT NULL check("index" >= 0) UNIQUE;