TARGET=$(shell cat target.txt)

.PHONY: lint
lint:
	@cross clippy

.PHONY: fix
fix:
	@cross clippy --fix

.PHONY: test
test:
	@cross test

.PHONY: shellspec
shellspec:
	@shellspec

.PHONY: install
install:
	@cross install --path .

.PHONY: check
check:
	@cross check

.PHONY: target-add
target-add:
	@rustup override set stable
	@rustup target add $(TARGET)

.PHONY: target-build
target-build: target-add
	@for target in $(TARGET); do echo $$target; cross build --release --target $$target; done

.PHONY: target-archive
target-archive: target-build
	@bash archive.sh $(TARGET)
