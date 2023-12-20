CREATE TABLE links (
    url VARCHAR(1024) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    clicks INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (slug)
);


CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_links_updated_at BEFORE UPDATE
ON links FOR EACH ROW EXECUTE PROCEDURE
update_updated_at_column();
