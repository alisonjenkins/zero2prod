#!/usr/bin/env bash
set -eo pipefail

if ! command -v psql &>/dev/null; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! command -v sqlx &>/dev/null; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use: "
  echo >&2 "  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

if ! podman ps --filter "name=postgres" | grep 'Up' &>/dev/null; then
  set +e
  podman rm postgres
  set -e
  podman run -p "$DB_PORT:$DB_PORT" --name postgres -e POSTGRES_USER="$DB_USER" \
    -e POSTGRES_PASSWORD="$DB_PASSWORD" \
    -e POSTGRES_DB="$DB_NAME" \
    -e POSTGRES_PORT="$DB_PORT" \
    -d postgres \
    postgres -N 1000
fi

export PGPASSWORD="$DB_PASSWORD"
until psql -h localhost -U "$DB_USER" -p "$DB_PORT" -d postgres -c '\q' &>/dev/null; do
  >&2 echo "Postgres is still unavailable - sleeping..."
  sleep 1
done

>&2 echo "Postgres is up and running on port $DB_PORT!"

export DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
