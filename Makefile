include .env

.PHONY: migrate
migrate:
	docker compose exec -e DATABASE_URL="mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@mysql:3306/${MYSQL_DATABASE}" \
		app sqlx migrate run

# make create-migrate name=add_orders_table
.PHONY: create-migrate
create-migrate:
	docker compose exec app sqlx migrate add -r $(name)

.PHONY: build-prod
build-prod:
	docker build -t rust-ec-shop:latest -f Dockerfile . 