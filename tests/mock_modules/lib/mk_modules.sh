#!/bin/bash
set -euxo pipefail

set -x

: clean slate
rm -rv ./module-namespace* || true  # no such file

i=0  # a small pseudo-pseudo-random number
for namespace in a b c; do
  for name in x y z; do
    i=$(((i + 23) % 13))
    for version in \
      "$(((i % 5) + 1)).0.$((i))" \
      "$(((i % 5) + 1)).0.$((i + 1))" \
      "$(((i % 4) + 1)).1.$((i))" \
    ; do
      module="module-namespace-$namespace/module-name-$name/$version"
      mkdir -p "$module"
      cat > "$module/$name.tf" <<EOF
output "version" { value = "$version" }
output "namespace" { value = "$namespace" }
output "name" { value = "$name" }
EOF
      tar -cvf "$module.tar" "$module"
      xz "$module.tar"
      rm -r "$module"
    done
  done
done
