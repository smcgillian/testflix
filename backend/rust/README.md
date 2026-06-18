# TestFlix Rust Backend

Rust backend implementation using Axum + SQLx.

## Location

This service lives in [backend/rust](backend/rust) and does not mix with the Go backend.

## Requirements

- Rust toolchain (stable)
- PostgreSQL running locally
- Database created as `testflix`
- Schema and seed loaded from:
  - [db/schema.sql](db/schema.sql)
  - [db/seed.sql](db/seed.sql)

## Environment Variables

- `TESTFLIX_DB_HOST` (required)
- `TESTFLIX_DB_PORT` (required)
- `TESTFLIX_DB_NAME` (required)
- `TESTFLIX_DB_USER` (required)
- `TESTFLIX_DB_PASSWORD` (required)
- `TESTFLIX_PORT` (optional)
  - Default: `3000`
- `DB_MAX_CONNECTIONS` (optional)
  - Default: `10`
- `FRONTEND_DIR` (optional)
  - Default: `../../frontend`

Example:

```bash
export TESTFLIX_DB_HOST="your-rds-endpoint"
export TESTFLIX_DB_PORT=5432
export TESTFLIX_DB_NAME="testflix"
export TESTFLIX_DB_USER="postgres"
export TESTFLIX_DB_PASSWORD="your-password"
export TESTFLIX_PORT=3000
export DB_MAX_CONNECTIONS=10
```

## Run

From [backend/rust](backend/rust):

```bash
cargo run
```

Server listens on `0.0.0.0:3000` by default.

## API Base

- `http://localhost:3000/api`

Implemented endpoints:

- `GET /api/media?email={email}`
- `GET /api/media/{id}`
- `GET /api/services`
- `GET /api/customers/{email}`
- `GET /api/customers/{email}/subscriptions`
- `PUT /api/subscriptions/{id}`

## Quick Curl Checks

```bash
curl "http://localhost:3000/api/services"
curl "http://localhost:3000/api/media?email=james.wilson@example.com"
curl "http://localhost:3000/api/customers/james.wilson@example.com"
```

Update subscription example:

```bash
curl -X PUT "http://localhost:3000/api/subscriptions/1" \
  -H "Content-Type: application/json" \
  -d '{"service_id":2,"billing_period":1}'
```

## Notes

- Response shape matches existing backend contract:
  - success: `{ "data": ... }`
  - error: `{ "error": "..." }`
- No tests were added yet (per current request).
- No Valkey integration is included yet.
