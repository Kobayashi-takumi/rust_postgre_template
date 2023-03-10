build:
	docker compose build
	docker compose run --rm app sh -c "cargo build"
up:
	docker compose up -d
down:
	docker compose down --rmi all --volumes --remove-orphans
ps:
	docker compose ps
log:
	docker compose logs -f
sh:
	docker compose run --rm app sh -c "$(ARGS)"
init:
	docker compose exec app bash
client:
	docker compose run --rm app sh -c "psql -U sa -W -h postgres -d template"
migrate-add:
	docker compose run --rm app sh -c "sqlx migrate add $(ARGS)"
migrate-run:
	docker compose run --rm app sh -c "sqlx migrate run"