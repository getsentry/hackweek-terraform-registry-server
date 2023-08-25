module "test" {
  source = "127.0.0.1:8555/namespace_a/module_x/system2"
  version = "~> 2.0"  # should pick: 2.1.21
}
