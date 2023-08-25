#!/bin/bash
set -euxo pipefail

set -x

: clean slate
rm -rv ./namespace_* || true  # no such file

i=0  # a small pseudo-pseudo-random number
for namespace in a b c; do
  for module in x y z; do
    for system in \
        $(((i + 7) % 3 + 1)) \
        $(((i + 5) % 2 + 1)) \
    ; do
      i=$(((i + 51) % 31 + 1))
      for version in \
        "$(((i % 5) + 1)).0.$((i))" \
        "$(((i % 5) + 1)).0.$((i + 1))" \
        "$(((i % 4) + 1)).1.$((i))" \
      ; do
        moduledir="namespace_$namespace/module_$module/system_$system/$version"
        mkdir -p "$moduledir"
        cat > "$moduledir/$module.tf" <<EOF
output "namespace" { value = "$namespace" }
output "module" { value = "$module" }
output "system" { value = "$system" }
output "version" { value = "$version" }
EOF
        tar -cvf "$moduledir.tar" "$moduledir"
        xz "$moduledir.tar" || true  # file already exists
        rm -r "$moduledir"
      done
    done
  done
done
