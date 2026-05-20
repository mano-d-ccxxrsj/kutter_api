# Kutter API

This document explains the current project structure, how to run the API, and where to change code when adding features.

Repository: [mano-d-ccxxrsj/kutter_api](https://github.com/mano-d-ccxxrsj/kutter_api)

Language: [Português](https://github.com/mano-d-ccxxrsj/kutter_api/blob/main/docs/pt-BR/README.md)

## Overview

The project is a Rust workspace split by responsibility. The current direction is to keep business rules explicit in the domain, treat database and HTTP as adapters, and centralize concrete application wiring in the `app` crate.

## Structure

| Path          | Responsibility                                                                                                     |
|---------------|--------------------------------------------------------------------------------------------------------------------|
| `app`         | Entry point, Tokio runtime creation, initial schema setup, and concrete service composition.                       |
| `domain`      | Entities, commands, errors, ports, and business services. It should know contracts, not Postgres or Actix details. |
| `persistence` | Postgres implementation of repository ports, SQLx models, mappers, schema, and pool.                               |
| `web`         | Actix server, routes, handlers, and request/response DTOs.                                                         |
| `infra`       | Environment loading and default configuration values.                                                              |
| `security`    | Bcrypt, JWT, and public key generation.                                                                            |
| `shared`      | Types and ports shared by more than one layer.                                                                     |

## Dependency Flow

`domain` defines what the application needs. `persistence`, `security`, and `web` implement external details. `app` connects everything through `ApplicationConfig`.

The expected flow is:

```text
web -> domain <- persistence
app -> web/domain/persistence/security/infra/shared
```

The `domain` crate must not depend on `web`, `persistence`, `infra`, or `security`.

## Environment

The application reads `.env` automatically through `infra::EnvConfig`. `JWT_KEY` is required.

| Variable                     | Purpose                                                                                                     |
|------------------------------|-------------------------------------------------------------------------------------------------------------|
| `APP_HOST`                   | API host outside Docker. Default: `127.0.0.1`.                                                              |
| `APP_CLIENT_PORT`            | Expected client/frontend port. Default: `3001`.                                                             |
| `APP_SERVER_PORT`            | API HTTP port. Default: `8080`.                                                                             |
| `USE_HTTPS`                  | Enables/disables logical HTTPS configuration. Default: `false`.                                             |
| `JWT_KEY`                    | Key used to sign session tokens. Required.                                                                  |
| `CONTENT_MODERATION_ENABLED` | Enables/disables content filtering. Default: `false`.                                                       |
| `DB_HOST`                    | Postgres host. Use `localhost` on the host; inside Docker the app switches it to `db_postgres` when needed. |
| `DB_PORT`                    | Postgres port. Default: `5432`.                                                                             |
| `DB_USER`                    | Postgres user.                                                                                              |
| `DB_PASSWORD`                | Postgres password.                                                                                          |
| `DB_NAME`                    | Database name.                                                                                              |
| `DATABASE_URL`               | Optional. When set, replaces the URL built from the `DB_*` variables.                                       |
| `DB_MAX_CONNECTIONS`         | Maximum pool connections.                                                                                   |
| `DB_MIN_CONNECTIONS`         | Minimum pool connections.                                                                                   |
| `DB_ACQUIRE_TIMEOUT`         | Connection acquire timeout, in seconds.                                                                     |
| `DB_MAX_LIFETIME`            | Maximum connection lifetime, in seconds.                                                                    |
| `DB_IDLE_TIMEOUT`            | Maximum idle connection time, in seconds.                                                                   |

## Running with Docker

To run the API and Postgres:

```bash
docker compose up --build
```

To run only the database and start the API on the host:

```bash
docker compose up -d db_postgres
```

Followed by:

```bash
cargo run -p app
```

The current schema is created on application startup by functions in `persistence/src/database/schema.rs`.

## Running on the Host

Requirements:

- Rust installed.
- Postgres running locally.
- A `.env` file with `JWT_KEY` and database variables.

With the database available:

```bash
cargo run -p app
```

To validate compilation and tests:

```bash
cargo check
```

Followed by:

```bash
cargo test
```

## Content Moderation

Content moderation is configured with `CONTENT_MODERATION_ENABLED`.

When disabled, `ToggleContentModerationService` allows content without querying banned terms. When enabled, the service reads `banned_words`, normalizes content, and records violations in `user_flags`.

Until administrative CRUD exists, terms can be inserted directly:

```sql
INSERT INTO banned_words (word, active) VALUES ('example', TRUE);
```

## Adding a Feature

1. Start in `domain`.
2. Create or adjust entities in `domain/src/entities`.
3. Create input/output types in `domain/src/types` when the operation needs its own commands.
4. Define ports in `domain/src/ports`.
5. Implement the rule in `domain/src/services`.
6. If the feature uses the database, implement model, mapper, and repository in `persistence`.
7. Update `persistence/src/database/schema.rs` if the feature needs a table or column.
8. If the feature has HTTP, create manual DTOs in `web/src/dto` and handlers in `web/src/handlers`.
9. Register routes in `web/src/http/server.rs`.
10. Wire the concrete implementation in `app/src/config/application_config.rs`.
11. Run `cargo check` and `cargo test`.

## Current Conventions

- Prefer explicit types for relevant variables.
- Prefer manual DTOs over serialization derives when control matters.
- Keep aliases, structs, and services in files that represent their responsibility.
- Avoid spreading configuration or composition strings outside `infra` and `app`.
- Use macros only when crate integration justifies them, such as `async_trait` and `sqlx::FromRow`.

## Current Endpoints

| Method | Path                          | Purpose                                            |
|--------|-------------------------------|----------------------------------------------------|
| `POST` | `/api/user/register`          | Registers a user.                                  |
| `POST` | `/api/user/login`             | Authenticates a user and creates a session cookie. |
| `POST` | `/api/community/create`       | Creates an authenticated community.                |
| `POST` | `/api/channel/create`         | Creates an authenticated channel.                  |
| `POST` | `/api/channel/message/send`   | Sends a message.                                   |
| `POST` | `/api/channel/message/list`   | Lists channel messages.                            |
| `POST` | `/api/channel/message/edit`   | Edits a message owned by the author.               |
| `POST` | `/api/channel/message/delete` | Deletes a message owned by the author.             |