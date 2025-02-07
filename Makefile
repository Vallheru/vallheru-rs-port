install-deps:
	cargo install cargo-generate --features vendored-openssl
	cargo install trunk leptosfmt
	cargo install cargo-leptos
	cargo install cargo-watch
	rustup target add wasm32-unknown-unknown
	npm install -D tailwindcss
	
frontend-serve:
	cd frontend && trunk serve

frontend-build:
	cd frontend && trunk build --release

backend-build:
	cargo build --bin backend

backend-watch:
	cargo watch -q -c -w backend/src/ -x "run --bin backend"

backend-quick-dev-watch:
	cd backend && cargo watch -q -c -w examples/ -x "run --package backend --example quick_dev"

db-migrate:
	PGPASSWORD=vallheru123 psql --host localhost --user vallheru --dbname vallheru_db -a -f ./backend/migrations/0_init.sql
