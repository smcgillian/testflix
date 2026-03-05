Overview

This document defines a compact design for a REST CRUD demo focused on a streaming-service domain. The goal is a runnable demo showing how to model services, customers, and streamable media, backed by PostgreSQL and exposed via a Go HTTP API.

Domain summary

- TestFlix: a logical grouping representing the platform or offering,
- Service: subscription tiers the platform offers (e.g., `premium`, `standard`). A `Service` has a `type` (premium or standard/ad-supported).
- Media: a streamable item that belongs to a `Service`. Each media has a `type` — currently `movie` or `tvshow` (more types like `music` may be added later).
- Customer: end-user records; customers subscribe to Services. If a customer has a subscription to a Service, they get access to that Service's Media.

Key rules

- Media may belong to one or more `Service` tiers (many-to-many relationship).
- Customers must have exactly one subscription (chosen at account creation). They can change their tier but not remove it.
- `premium` grants full access; `standard` is ad-supported (for the demo this is a type flag).

Architecture

- Backend: Go HTTP server exposing JSON REST endpoints.
- Database: PostgreSQL storing `services`, `media`, `customers`, and `subscriptions`.
- Spec: `proto/streaming.proto` for data contracts.

DB Schema

See `db/schema.sql` for full DDL. Summary of tables:

- `services` — subscription tiers (premium, standard)
- `media_types` — lookup: movie, tvshow
- `classifications` — age ratings: U, PG, PG-13, R
- `media` — streamable items (title, description, duration, genre, FK to media_type and classification)
- `media_services` — many-to-many mapping of media to services
- `customers` — end users (first_name, last_name, email; email is the unique login identifier)
- `subscriptions` — which customers subscribe to which services (billing_period, started_at, expires_at)

Entity-Relationship Diagram

```
┌──────────────┐       ┌─────────────────┐       ┌───────────────┐
│  media_types │       │     media       │       │classifications│
├──────────────┤       ├─────────────────┤       ├───────────────┤
│ id       PK  │───┐   │ id          PK  │   ┌───│ id       PK   │
│ name         │   │   │ title           │   │   │ name          │
└──────────────┘   │   │ media_type_id FK│◄──┘   └───────────────┘
                   └──►│ description     │
                       │ duration_seconds│
                       │ genre           │
                       │ classification_id FK│
                       └────────┬────────┘
                                │
                                │ 1
                                │
                                │ *
                       ┌────────┴────────┐
                       │ media_services  │
                       ├─────────────────┤
                       │ media_id  PK,FK │
                       │ service_id PK,FK│
                       └────────┬────────┘
                                │
                                │ *
                                │
                                │ 1
                       ┌────────┴────────┐
                       │    services     │
                       ├─────────────────┤
                       │ id          PK  │
                       │ name            │
                       │ type            │
                       └────────┬────────┘
                                │
                                │ 1
                                │
                                │ *
                       ┌────────┴────────┐
                       │  subscriptions  │
                       ├─────────────────┤
                       │ id          PK  │
                       │ customer_id FK  │
                       │ service_id  FK  │
                       │ started_at      │
                       │ expires_at      │
                       │ billing_period  │
                       └────────┬────────┘
                                │
                                │ *
                                │
                                │ 1
                       ┌────────┴────────┐
                       │   customers     │
                       ├─────────────────┤
                       │ id          PK  │
                       │ first_name      │
                       │ last_name       │
                       │ email    UNIQUE │
                       └─────────────────┘

Relationships:
  media_types    1──* media           (each media has one type)
  classifications 1──* media          (each media has one classification)
  media          1──* media_services  (media can be on multiple services)
  services       1──* media_services  (services can have multiple media)
  services       1──* subscriptions   (services can have multiple subscribers)
  customers      1──* subscriptions   (customers can have multiple subscriptions)
```

Seed data

See `db/seed.sql` for full INSERT statements. Summary:

- 2 services: TestFlix Premium, TestFlix Standard
- 4 classifications: U, PG, PG-13, R
- 2 media types: movie, tvshow
- 30 media items: 20 movies, 10 TV shows
- Media-service mapping: all 30 on premium; 10 movies + 5 TV shows on standard (premium is a superset)
- 5 customers: James Wilson, Sarah Chen, Marcus Rivera, Emily Hart, David Kim
- 5 subscriptions: james/marcus/david → premium, sarah/emily → standard

Access rules (demo behavior)

- All media endpoints require an `email` query parameter. If `email` is missing, return `400 Bad Request`.
- The server looks up the customer's subscriptions and returns only media belonging to those services.
- Every customer has exactly one active subscription, so the media list is never empty.
- If the `email` does not match any customer, return `404 Not Found`.
- There is no public/anonymous media access.

API Endpoints

All API endpoints are served under the `/api/` prefix. The Go server also serves
static frontend files from the `frontend/` directory at the root path.

The API serves a customer-facing frontend. Admin operations (creating media,
managing service tiers, populating lookup tables) are handled via seed data and
Go tooling, not exposed as REST endpoints.

---

### GET /api/media?email={email}

List media the authenticated user can access based on their subscriptions.

Query parameters:
- `email` (required) — customer email address

Logic:
1. Look up customer by email → 404 if not found
2. Look up customer's subscriptions → get subscribed service IDs
3. Query media joined through media_services for those service IDs
4. Return matched media (customer always has a subscription)

Response `200`:
```json
{
  "data": [
    {
      "id": 1,
      "title": "Galactic Odyssey",
      "media_type": "movie",
      "description": "Sci-fi feature film",
      "duration_seconds": 7200,
      "genre": "sci-fi",
      "classification": "PG-13"
    }
  ]
}
```

Errors:
- `400` — `{ "error": "email query parameter is required" }`
- `404` — `{ "error": "customer not found" }`

---

### GET /api/media/{id}

Get a single media item by ID. No subscription check — any known media ID is accessible.

Response `200`:
```json
{
  "data": {
    "id": 1,
    "title": "Galactic Odyssey",
    "media_type": "movie",
    "description": "Sci-fi feature film",
    "duration_seconds": 7200,
    "genre": "sci-fi",
    "classification": "PG-13",
    "services": ["TestFlix Premium"]
  }
}
```

Errors:
- `404` — `{ "error": "media not found" }`

---

### GET /api/services

List all available service tiers.

Response `200`:
```json
{
  "data": [
    { "id": 1, "name": "TestFlix Premium", "type": "premium" },
    { "id": 2, "name": "TestFlix Standard", "type": "standard" }
  ]
}
```

---

### GET /api/customers/{email}

Get a customer profile by email address.

Response `200`:
```json
{
  "data": {
    "id": 1,
    "first_name": "James",
    "last_name": "Wilson",
    "email": "james.wilson@example.com"
  }
}
```

Errors:
- `404` — `{ "error": "customer not found" }`

---

### GET /api/customers/{email}/subscriptions

List all subscriptions for a customer.

Response `200`:
```json
{
  "data": [
    {
      "id": 1,
      "customer_id": 1,
      "service_id": 1,
      "service_name": "TestFlix Premium",
      "service_type": "premium",
      "billing_period": 1,
      "started_at": "2026-03-04T21:09:46Z",
      "expires_at": null
    }
  ]
}
```

Errors:
- `404` — `{ "error": "customer not found" }`

---

### PUT /api/subscriptions/{id}

Change an existing subscription to a different service tier.

Request body:
```json
{
  "service_id": 2,
  "billing_period": 1
}
```

Validation:
- `id` must reference an existing subscription
- `service_id` must reference an existing service
- `billing_period` must be `1` (monthly) or `2` (yearly)

Logic:
- Updates the subscription's `service_id` and `billing_period`
- Resets `started_at` to now

Response `200`:
```json
{
  "data": {
    "id": 3,
    "customer_id": 3,
    "service_id": 2,
    "billing_period": 1,
    "started_at": "2026-03-05T10:00:00Z",
    "expires_at": null
  }
}
```

Errors:
- `400` — `{ "error": "invalid request body" }` or `{ "error": "billing_period must be 1 or 2" }`
- `404` — `{ "error": "subscription not found" }` or `{ "error": "service not found" }`

---

Status codes summary: `200` success, `400` bad request, `404` not found, `500` server error.

Protobuf data definitions (proto/streaming.proto)

```proto
syntax = "proto3";
package streaming;
option go_package = "github.com/testflix/proto;proto";

message Service {
  int64 id = 1;
  string name = 2;
  string type = 3; // "premium" or "standard"
}

message Media {
  int64 id = 1;
  string title = 2;
  int32 media_type_id = 3; // FK to media_types: 1=movie, 2=tvshow
  string description = 4;
  int32 duration_seconds = 5;
  string genre = 6;
  int32 classification_id = 7; // FK to classifications: 1=U, 2=PG, 3=PG-13, 4=R
}

message Customer {
  int64 id = 1;
  string first_name = 2;
  string last_name = 3;
  string email = 4;
}

message Subscription {
  int64 id = 1;
  int64 customer_id = 2;
  int64 service_id = 3;
  string started_at = 4; // ISO timestamp
  string expires_at = 5; // ISO timestamp
  int32 billing_period = 6; // 1=monthly, 2=yearly
}
```

SQL files

- `db/schema.sql` — DDL: all CREATE TABLE statements and constraints. Run once to set up the database structure.
- `db/seed.sql` — DML: INSERT statements with example services, media, customers, and subscriptions. Run after schema to populate test data.

Implementation notes

- Scope is a compact, production-style full-stack application.
- Use `github.com/jackc/pgx/v5` with its stdlib-compatible driver (`pgx/v5/stdlib`) so we get `database/sql` compatibility plus pgx performance.
- Router: Go standard library `net/http.ServeMux` (Go 1.22+ enhanced routing with method and path-parameter support). No third-party router needed — fewer dependencies, stdlib stability guarantees, and what production-grade Go shops default to.
- The Go server listens on `:8080`, serving both the API (`/api/...`) and static frontend files (`/`).
- Authentication is out of scope for now — APIs accept `email` as a parameter to identify the acting customer for demo flows. OAuth will be added later.

Front End

### Stage 1 — Plain JavaScript

No frameworks. Vanilla JS with `fetch` against the REST API. Pages are separate HTML files served statically.

#### Pages

1. **Login** (`login.html`) — Email address input, submit button. On submit, validate the email exists via `GET /customers/{email}`. If 404, show error. If 200, store email and first name in `sessionStorage` and redirect to the media list.

2. **Media List** (`index.html`) — Fetch `GET /media?email={email}` and render a list of titles with media type, genre, and classification. Each title links to the media detail page.

3. **Media Detail** (`media.html?id={id}`) — Fetch `GET /media/{id}` and display title, description, duration, genre, classification, and which services carry it. Include a "Play" button (disabled/non-functional for this demo).

4. **User Profile** (`profile.html`) — Fetch `GET /customers/{email}` and `GET /customers/{email}/subscriptions`. Display "Hello [First Name]", full name, email, and current subscription. Allow the user to:
   - Change their subscription to a different service tier (`PUT /subscriptions/{id}`)
   
   Changes take effect immediately — returning to the media list reflects the updated catalogue.

5. **Navigation** — Every page (except login) includes a header with:
   - "TestFlix" logo/text linking to the media list
   - "Account" link to the user profile page
   - "Logout" link that clears `sessionStorage` and returns to login

#### Site Map

```
                    ┌─────────────┐
                    │  Login      │
                    │ login.html  │
                    └──────┬──────┘
                           │ submit username
                           ▼
                    ┌─────────────┐
               ┌───►│ Media List  │◄───┐
               │    │ index.html  │    │
               │    └──────┬──────┘    │
               │           │ click     │
               │           ▼ title     │
               │    ┌─────────────┐    │
               │    │Media Detail │    │
               │    │ media.html  │    │
               │    └─────────────┘    │
               │                       │
               │    ┌─────────────┐    │
               └────│User Profile │────┘
           account  │profile.html │  back to
            link    └─────────────┘  titles
```

Navigation flow:
- Login → Media List (on successful username entry)
- Media List → Media Detail (click a title)
- Media Detail → Media List (back / logo)
- Media List, Media Detail → User Profile (account link)
- User Profile → Media List (after subscription change or back)
- Media List, Media Detail, User Profile → Login (logout)

### Stage 2 — React

Once the plain JS frontend is working correctly end-to-end, migrate to React. Same pages, same API calls, component-based structure.

Next steps

- Scaffold `main.go` handlers to implement the endpoints above.
