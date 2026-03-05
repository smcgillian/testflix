-- reset.sql — Drop all tables for testflix
-- Usage: psql -d testflix -f db/reset.sql

DROP TABLE IF EXISTS subscriptions;
DROP TABLE IF EXISTS media_services;
DROP TABLE IF EXISTS media;
DROP TABLE IF EXISTS classifications;
DROP TABLE IF EXISTS media_types;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS services;
