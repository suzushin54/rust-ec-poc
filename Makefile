include .env

.PHONY: migrate
migrate:
	docker compose exec -e DATABASE_URL="mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@mysql:3306/${MYSQL_DATABASE}" \
		app sqlx migrate run

.PHONY: build-prod
build-prod:
	docker build -t rust-ec-shop:latest -f Dockerfile . 