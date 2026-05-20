use sqlx::{PgPool, postgres::PgQueryResult};

pub async fn create_auth_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    create_users_table(pool).await?;
    migrate_users_table(pool).await?;
    create_tokens_table(pool).await?;
    migrate_tokens_table(pool).await?;

    Ok(())
}

pub async fn create_community_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    create_communities_table(pool).await?;
    migrate_communities_table(pool).await?;
    create_members_table(pool).await?;
    migrate_members_table(pool).await?;
    create_channels_table(pool).await?;
    migrate_channels_table(pool).await?;
    create_messages_table(pool).await?;
    migrate_messages_table(pool).await?;

    Ok(())
}

pub async fn create_content_moderation_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    create_banned_words_table(pool).await?;
    migrate_banned_words_table(pool).await?;
    create_user_flags_table(pool).await?;
    migrate_user_flags_table(pool).await?;

    Ok(())
}

async fn create_users_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR(30)  NOT NULL UNIQUE,
            email           TEXT         NOT NULL UNIQUE,
            password_hash   VARCHAR(255) NOT NULL,
            public_key      BYTEA        NOT NULL UNIQUE,
            verified        BOOLEAN      NOT NULL DEFAULT FALSE,
            profile_picture TEXT,
            biography       VARCHAR(200),
            created_at      TIMESTAMPTZ  NOT NULL,
            updated_at      TIMESTAMPTZ  NOT NULL
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_users_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query(
        "DO $$
        BEGIN
            IF EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'users' AND column_name = 'password'
            ) AND NOT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'users' AND column_name = 'password_hash'
            ) THEN
                ALTER TABLE users RENAME COLUMN password TO password_hash;
            END IF;

            IF EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'users' AND column_name = 'pub_key'
            ) AND NOT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'users' AND column_name = 'public_key'
            ) THEN
                ALTER TABLE users RENAME COLUMN pub_key TO public_key;
            END IF;
        END $$;",
    )
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS username VARCHAR(30)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS email TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS public_key BYTEA")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS verified BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS profile_picture TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS biography VARCHAR(200)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE users SET created_at = NOW() WHERE created_at IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ALTER COLUMN created_at SET NOT NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE users SET updated_at = created_at WHERE updated_at IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE users ALTER COLUMN updated_at SET NOT NULL")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_tokens_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tokens (
            id          SERIAL PRIMARY KEY,
            user_id     INTEGER     NOT NULL REFERENCES users(id),
            created_at  TIMESTAMPTZ NOT NULL,
            last_update TIMESTAMPTZ NOT NULL,
            revoked     BOOLEAN     NOT NULL DEFAULT FALSE,
            revoked_at  TIMESTAMPTZ
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_tokens_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ADD COLUMN IF NOT EXISTS user_id INTEGER REFERENCES users(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE tokens SET created_at = NOW() WHERE created_at IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ALTER COLUMN created_at SET NOT NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ADD COLUMN IF NOT EXISTS last_update TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE tokens SET last_update = created_at WHERE last_update IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ALTER COLUMN last_update SET NOT NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ADD COLUMN IF NOT EXISTS revoked BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE tokens ADD COLUMN IF NOT EXISTS revoked_at TIMESTAMPTZ")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_communities_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS communities (
            id    SERIAL PRIMARY KEY,
            name  VARCHAR(50) NOT NULL,
            about TEXT,
            nsfw  BOOLEAN     NOT NULL
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_communities_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE communities ADD COLUMN IF NOT EXISTS name VARCHAR(50)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE communities ADD COLUMN IF NOT EXISTS about TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE communities ADD COLUMN IF NOT EXISTS nsfw BOOLEAN")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE communities SET nsfw = FALSE WHERE nsfw IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE communities ALTER COLUMN nsfw SET NOT NULL")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_members_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS members (
            id           SERIAL PRIMARY KEY,
            user_id      INTEGER NOT NULL REFERENCES users(id),
            community_id INTEGER NOT NULL REFERENCES communities(id),
            owner        BOOLEAN NOT NULL DEFAULT FALSE,
            admin        BOOLEAN NOT NULL DEFAULT FALSE
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_members_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE members ADD COLUMN IF NOT EXISTS user_id INTEGER REFERENCES users(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE members ADD COLUMN IF NOT EXISTS community_id INTEGER REFERENCES communities(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE members ADD COLUMN IF NOT EXISTS owner BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE members ADD COLUMN IF NOT EXISTS admin BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_channels_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS channels (
            id           SERIAL PRIMARY KEY,
            community_id INTEGER      NOT NULL REFERENCES communities(id),
            name         VARCHAR(100) NOT NULL,
            topic        TEXT,
            hidden       BOOLEAN      NOT NULL DEFAULT FALSE
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_channels_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query(
        "DO $$
        BEGIN
            IF EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'channels' AND column_name = 'community'
            ) AND NOT EXISTS (
                SELECT 1 FROM information_schema.columns
                WHERE table_name = 'channels' AND column_name = 'community_id'
            ) THEN
                ALTER TABLE channels RENAME COLUMN community TO community_id;
            END IF;
        END $$;",
    )
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE channels ADD COLUMN IF NOT EXISTS community_id INTEGER REFERENCES communities(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE channels ADD COLUMN IF NOT EXISTS name VARCHAR(100)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE channels ADD COLUMN IF NOT EXISTS topic TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE channels ADD COLUMN IF NOT EXISTS hidden BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_messages_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id              SERIAL PRIMARY KEY,
            channel_id      INTEGER     NOT NULL REFERENCES channels(id),
            user_id         INTEGER     NOT NULL REFERENCES users(id),
            message         TEXT        NOT NULL,
            replied_message INTEGER     REFERENCES messages(id),
            timestamp       TIMESTAMPTZ NOT NULL,
            edited          BOOLEAN     NOT NULL DEFAULT FALSE
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_messages_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS channel_id INTEGER REFERENCES channels(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS user_id INTEGER REFERENCES users(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS message TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS replied_message INTEGER REFERENCES messages(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS timestamp TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE messages SET timestamp = NOW() WHERE timestamp IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ALTER COLUMN timestamp SET NOT NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ADD COLUMN IF NOT EXISTS edited BOOLEAN DEFAULT FALSE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE messages SET edited = FALSE WHERE edited IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ALTER COLUMN edited SET DEFAULT FALSE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE messages ALTER COLUMN edited SET NOT NULL")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_banned_words_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS banned_words (
            id     SERIAL PRIMARY KEY,
            word   TEXT    NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT TRUE
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_banned_words_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE banned_words ADD COLUMN IF NOT EXISTS word TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE banned_words ADD COLUMN IF NOT EXISTS active BOOLEAN DEFAULT TRUE")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE banned_words SET active = TRUE WHERE active IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE banned_words ALTER COLUMN active SET NOT NULL")
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_user_flags_table(pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_flags (
            id             SERIAL PRIMARY KEY,
            user_id        INTEGER     NOT NULL REFERENCES users(id),
            field          TEXT        NOT NULL,
            action         TEXT        NOT NULL,
            target         TEXT        NOT NULL,
            attempted_text TEXT        NOT NULL,
            matched_words  TEXT        NOT NULL,
            details        TEXT,
            created_at     TIMESTAMPTZ NOT NULL
        )",
    )
        .execute(pool)
        .await
}

async fn migrate_user_flags_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS user_id INTEGER REFERENCES users(id)")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS field TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS action TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS target TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS attempted_text TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS matched_words TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS details TEXT")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("UPDATE user_flags SET created_at = NOW() WHERE created_at IS NULL")
        .execute(pool)
        .await?;

    let _: PgQueryResult = sqlx::query("ALTER TABLE user_flags ALTER COLUMN created_at SET NOT NULL")
        .execute(pool)
        .await?;

    Ok(())
}