.PHONY: all
all: devenv
	@echo $$'\033[32mSuccess\033[m: Let'\''s go!'

.Brewfile.done: Brewfile
	@makerule $@ brew bundle

.PHONY: devenv
devenv: .Brewfile.done
	make -C ./tests/mock_modules


localhost+1.pem:
	mkcert -install
	@makerule $@ mkcert localhost 127.0.0.1

serve:
	cargo run
