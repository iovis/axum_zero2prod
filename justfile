set dotenv-load := true

default: run

# lists available tasks
@list:
    just --list

run:
    cargo run

build:
    cargo build

dev:
    cargo watch -x clippy -x run

console:
    evcxr

open:
    gh repo view --web

clean:
    cargo clean

# run tests
test:
    cargo nextest run

# Open the DB
db:
    litecli $DATABASE_URL

db_reset:
    sqlx database reset -y

db_migrate: db_create
    sqlx migrate run

db_create:
    sqlx database create
