module "a-x-2" {
  source  = "127.0.0.1:8443/namespace_a/module_x/system2"
  version = "~> 2.0" # should pick: 2.1.21
}

output "a-x-2" {
  value = module.a-x-2
}
