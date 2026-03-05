-- schema.sql — DDL for testflix
-- Usage: psql -d testflix -f db/schema.sql

CREATE TABLE services (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  type VARCHAR(20) NOT NULL CHECK (type IN ('premium','standard'))
);

CREATE TABLE media_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR(20) NOT NULL UNIQUE -- 'movie', 'tvshow'
);

CREATE TABLE classifications (
  id SERIAL PRIMARY KEY,
  name VARCHAR(10) NOT NULL UNIQUE -- 'U', 'PG', 'PG-13', 'R'
);

CREATE TABLE media (
  id SERIAL PRIMARY KEY,
  title VARCHAR(200) NOT NULL,
  media_type_id INT NOT NULL REFERENCES media_types(id),
  description VARCHAR(500) NULL,
  duration_seconds INT NULL,
  genre VARCHAR(50) NULL,
  classification_id INT NULL REFERENCES classifications(id)
);

CREATE TABLE media_services (
  media_id INT NOT NULL REFERENCES media(id) ON DELETE CASCADE,
  service_id INT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
  PRIMARY KEY (media_id, service_id)
);

CREATE TABLE customers (
  id SERIAL PRIMARY KEY,
  username VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE subscriptions (
  id SERIAL PRIMARY KEY,
  customer_id INT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
  service_id INT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
  started_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  expires_at TIMESTAMP WITH TIME ZONE NULL,
  billing_period SMALLINT NOT NULL, -- 1=monthly, 2=yearly
  UNIQUE (customer_id, service_id)
);