.PHONY: fetch check test clippy fmt fmt-check docker-shell docker-fetch docker-check docker-test docker-clippy docker-fmt docker-fmt-check docker-clean

fetch:
	cargo fetch

check:
	cargo check --all-targets

test:
	cargo test --all-targets

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

docker-shell:
	docker compose run --rm dev

docker-fetch:
	docker compose run --rm dev cargo fetch

docker-check:
	docker compose run --rm dev cargo check --all-targets

docker-test:
	docker compose run --rm dev cargo test --all-targets

docker-clippy:
	docker compose run --rm dev cargo clippy --all-targets -- -D warnings

docker-fmt:
	docker compose run --rm dev cargo fmt

docker-fmt-check:
	docker compose run --rm dev cargo fmt -- --check

docker-clean:
	docker compose down --volumes --remove-orphans
