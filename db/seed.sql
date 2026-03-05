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

-- Media: 30 titles (20 movies, 10 tvshows)
INSERT INTO media (title, media_type_id, description, duration_seconds, genre, classification_id) VALUES
  -- Movies (20)
  ('Galactic Odyssey',       1, 'Epic sci-fi space adventure',                    7200, 'sci-fi',      3),
  ('The Great Heist',        1, 'Action-packed bank robbery thriller',            6600, 'action',      4),
  ('Midnight in Paris',      1, 'Romantic drama set in Paris',                   6900, 'drama',       3),
  ('Ocean Adventures',       1, 'Family-friendly animated ocean film',            5100, 'animation',   1),
  ('Shadow Protocol',        1, 'Espionage action thriller',                      7800, 'action',      3),
  ('The Last Frontier',      1, 'Epic western set on the frontier',               8400, 'western',     4),
  ('Neon Requiem',           1, 'Neo-noir crime thriller',                        7500, 'thriller',    4),
  ('The Wandering Chef',     1, 'Comedy about a chef crossing Europe',            6300, 'comedy',      2),
  ('Iron Meridian',          1, 'Hard sci-fi near-future conspiracy',             8100, 'sci-fi',      3),
  ('Crimson Shore',          1, 'Murder mystery on a remote island',              6600, 'mystery',     3),
  ('Sunfall',                1, 'Post-apocalyptic survival drama',                7200, 'drama',       2),
  ('The Hollow Crown',       1, 'Historical epic of medieval power',              9000, 'historical',  3),
  ('Vortex',                 1, 'High-octane natural disaster action',            6900, 'action',      3),
  ('Paper Tigers',           1, 'Workplace comedy in a failing startup',          5700, 'comedy',      2),
  ('Dead of Winter',         1, 'Supernatural horror in a snowbound town',        6600, 'horror',      4),
  ('The Quiet Storm',        1, 'Character study of a jazz musician',             7800, 'drama',       2),
  ('Polar Drift',            1, 'Arctic survival adventure',                      7200, 'adventure',   2),
  ('Broken Frequency',       1, 'Psychological thriller about a radio DJ',        6900, 'thriller',    4),
  ('The Glass Menagerie',    1, 'Intimate family drama adapted from stage',       7500, 'drama',       2),
  ('Silverline',             1, 'Sci-fi road movie across a dying planet',        8400, 'sci-fi',      3),
  -- TV Shows (10)
  ('Night Shift',            2, 'Medical drama following ER nurses',              2700, 'drama',       3),
  ('Cosmic Bounty Hunters',  2, 'Animated sci-fi comedy series',                 1500, 'animation',   2),
  ('Street Kitchen',         2, 'Competitive cooking show',                       2400, 'reality',     1),
  ('Cold Case Files',        2, 'True crime docuseries',                          3000, 'documentary', 3),
  ('The Syndicate',          2, 'Gritty organised crime drama',                  3600, 'crime',       4),
  ('Mind the Gap',           2, 'Ensemble workplace comedy series',              1800, 'comedy',      2),
  ('Outbreak Protocol',      2, 'Pandemic thriller procedural',                  2700, 'thriller',    3),
  ('Starbound Academy',      2, 'Animated children''s space adventure',          1500, 'animation',   1),
  ('The Trading Floor',      2, 'Drama set in a high-stakes bank',               2700, 'drama',       3),
  ('Wild Tech',              2, 'Documentary series on frontier technology',      2400, 'documentary', 2);

-- Map media to services
-- Premium gets all 30 titles; standard gets 10 movies + 5 TV shows
INSERT INTO media_services (media_id, service_id)
  SELECT m.id, s.id FROM media m, services s WHERE s.type = 'premium';

INSERT INTO media_services (media_id, service_id)
  SELECT m.id, s.id FROM media m, services s
  WHERE s.type = 'standard'
  AND m.title IN (
    'Ocean Adventures', 'The Wandering Chef', 'Crimson Shore', 'Sunfall',
    'Vortex', 'Paper Tigers', 'The Quiet Storm', 'Polar Drift',
    'The Glass Menagerie', 'Silverline',
    'Street Kitchen', 'Cold Case Files', 'Mind the Gap', 'Starbound Academy', 'Wild Tech'
  );

-- Customers
INSERT INTO customers (first_name, last_name, email) VALUES
  ('James',  'Wilson',  'james.wilson@example.com'),
  ('Sarah',  'Chen',    'sarah.chen@example.com'),
  ('Marcus', 'Rivera',  'marcus.rivera@example.com'),
  ('Emily',  'Hart',    'emily.hart@example.com'),
  ('David',  'Kim',     'david.kim@example.com');

-- Subscriptions: james/marcus/david=premium, sarah/emily=standard
INSERT INTO subscriptions (customer_id, service_id, billing_period) VALUES
  ((SELECT id FROM customers WHERE email='james.wilson@example.com'),  (SELECT id FROM services WHERE type='premium'),  1),
  ((SELECT id FROM customers WHERE email='sarah.chen@example.com'),    (SELECT id FROM services WHERE type='standard'), 2),
  ((SELECT id FROM customers WHERE email='marcus.rivera@example.com'), (SELECT id FROM services WHERE type='premium'),  2),
  ((SELECT id FROM customers WHERE email='emily.hart@example.com'),    (SELECT id FROM services WHERE type='standard'), 1),
  ((SELECT id FROM customers WHERE email='david.kim@example.com'),     (SELECT id FROM services WHERE type='premium'),  1);