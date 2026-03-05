-- seed.sql — Test data for testflix
-- Usage: psql -d testflix -f db/seed.sql

-- Services
INSERT INTO services (name, type) VALUES
  ('TestFlix Premium', 'premium'),
  ('TestFlix Standard', 'standard');

-- Media types
INSERT INTO media_types (id, name) VALUES
  (1, 'movie'),
  (2, 'tvshow');

-- Classifications (age ratings)
INSERT INTO classifications (id, name) VALUES
  (1, 'U'),
  (2, 'PG'),
  (3, 'PG-13'),
  (4, 'R');

-- Media: 10 titles (6 movies, 4 tvshows)
INSERT INTO media (title, media_type_id, description, duration_seconds, genre, classification_id) VALUES
  ('Galactic Odyssey',       1, 'Sci-fi feature film',               7200, 'sci-fi',      3),
  ('The Great Heist',        1, 'Action-packed thriller',            6600, 'action',      4),
  ('Midnight in Paris',      1, 'Romantic drama set in Paris',       6900, 'drama',       3),
  ('Ocean Adventures',       1, 'Family-friendly animated film',     5100, 'animation',   1),
  ('Shadow Protocol',        1, 'Espionage action thriller',         7800, 'action',      3),
  ('The Last Frontier',      1, 'Western epic',                      8400, 'western',     4),
  ('Night Shift',            2, 'Medical drama series',              2700, 'drama',       3),
  ('Cosmic Bounty Hunters',  2, 'Animated sci-fi comedy',           1500, 'animation',   2),
  ('Street Kitchen',         2, 'Competitive cooking show',          2400, 'reality',     1),
  ('Cold Case Files',        2, 'True crime docuseries',             3000, 'documentary', 3);

-- Map media to services
-- Premium gets all 10 titles; standard gets 4 titles (premium is a superset)
INSERT INTO media_services (media_id, service_id) VALUES
  ((SELECT id FROM media WHERE title='Galactic Odyssey'),      (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='The Great Heist'),       (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Midnight in Paris'),     (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Shadow Protocol'),       (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='The Last Frontier'),     (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Night Shift'),           (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Ocean Adventures'),      (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Cosmic Bounty Hunters'), (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Street Kitchen'),        (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Cold Case Files'),       (SELECT id FROM services WHERE type='premium')),
  ((SELECT id FROM media WHERE title='Ocean Adventures'),      (SELECT id FROM services WHERE type='standard')),
  ((SELECT id FROM media WHERE title='Cosmic Bounty Hunters'), (SELECT id FROM services WHERE type='standard')),
  ((SELECT id FROM media WHERE title='Street Kitchen'),        (SELECT id FROM services WHERE type='standard')),
  ((SELECT id FROM media WHERE title='Cold Case Files'),       (SELECT id FROM services WHERE type='standard'));

-- Customers
INSERT INTO customers (username) VALUES ('alice'), ('bob'), ('carol');

-- Subscriptions: alice=premium monthly, bob=standard yearly, carol=standard monthly
INSERT INTO subscriptions (customer_id, service_id, billing_period) VALUES
  ((SELECT id FROM customers WHERE username='alice'), (SELECT id FROM services WHERE type='premium'), 1),
  ((SELECT id FROM customers WHERE username='bob'),   (SELECT id FROM services WHERE type='standard'), 2),
  ((SELECT id FROM customers WHERE username='carol'), (SELECT id FROM services WHERE type='standard'), 1);