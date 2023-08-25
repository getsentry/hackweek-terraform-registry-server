.PHONY: all
all: devenv
	@echo $$'\033[32mSuccess\033[m: Let'\''s go!'

.PHONY: devenv
devenv: brew cert
	make -C ./tests/mock_modules

.PHONY: brew
brew: .Brewfile.done
.Brewfile.done: Brewfile
	@makerule $@ brew bundle

.PHONY: cert
cert: tls/pem/127.0.0.1.pem
tls/pem/127.0.0.1.pem:
	mkcert -install
	# this is the layout expecgted by haproxy:
	mkcert -key-file tls/127.0.0.1.pem.key -cert-file tls/127.0.0.1.pem  127.0.0.1

.PHONY: serve
serve: devenv
	./bin/serve
