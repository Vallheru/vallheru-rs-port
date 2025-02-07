watch:
	cargo watch -q -c -w backend/src/ -x "xtask"

backend-quick-dev-watch:
	cargo watch -q -c -w examples/ -x "run --package backend --example quick_dev"

db-migrate:
	PGPASSWORD=vallheru123 psql --host localhost --user vallheru --dbname vallheru_db -a -f ./migrations/0_init.sql
