# Private Terraform Registry Server

## Features

- Implements the [Module Registry Protocol](https://developer.hashicorp.com/terraform/internals/module-registry-protocol) (`modules.v1`)
- Modules are stored on the filesystem

## Contributing

### Setup Your Dev Environment

```sh
make devenv
```

### Running the Server

```sh
make serve
```

### Running Tests

Unit tests can be ran with:

```sh
cargo test
```

Run integration tests with (start the server first with `make serve`):

```sh
./tests/integration/test
```
