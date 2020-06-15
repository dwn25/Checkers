APP=docker-compose exec app
DOCS=docker-compose exec docs

RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
CARGO_INCREMENTAL=0

.PHONY: app docs cov

all: fmt lint run

app:
	docker-compose up -d app

docs:
	docker-compose up -d docs

build: app
	$(APP) cargo build

test: app
	$(APP) cargo test

run: app
	$(APP) cargo run

watch: app
	$(APP) cargo watch -x test

fmt: app
	$(APP) cargo fmt

lint: app
	$(APP) cargo clippy

req-docs: docs
	$(DOCS) latexmk -cd -f -interaction=batchmode -pdf docs/requirements/index.tex

test-docs: docs
	$(DOCS) latexmk -cd -f -interaction=batchmode -pdf docs/test-cases/index.tex

final-docs: docs
	$(DOCS) latexmk -cd -f -interaction=batchmode -pdf docs/final/index.tex

src-docs: app
	$(APP) cargo doc --open

todo: app
	$(APP) grep -nEr 'XXX|TODO|FIXME' src

clean: docs app
	$(APP) cargo clean
	$(DOCS) latexmk -c -cd docs/requirements/index.tex
	$(DOCS) latexmk -c -cd docs/test-cases/index.tex
	$(DOCS) latexmk -c -cd docs/final/index.tex
	docker-compose down
	-rm -rf report lcov.info main.gcda main.gcno

cov:
	-mkdir report
	RUSTFLAGS=$(RUSTFLAGS) CARGO_INCREMENTAL=$(CARGO_INCREMENTAL) cargo build
	RUSTFLAGS=$(RUSTFLAGS) CARGO_INCREMENTAL=$(CARGO_INCREMENTAL) cargo test
	grcov target/debug -t lcov --branch > lcov.info
	lcov -e lcov.info src/* --rc lcov_branch_coverage=1 -o lcov.info
	cp -r src report
	genhtml -o report/ --show-details --highlight --ignore-errors source --legend --branch lcov.info
