#!/usr/bin/env bash
set -euo pipefail

echo "Regenerating mock modules..."
if [[ "${DEBUG:-}"  ]]; then
  set -x
  verbose=(-v)
else
  verbose=()
fi

: clean slate
rm -r "${verbose[@]}" ./namespace* || true  # no such file

i=0  # a small pseudo-pseudo-random number
for namespace in a b c; do
  namespace="namespace_$namespace"
  echo "$namespace/"
  for module in x y z; do
    module="module_$module"
    echo "  $module/"
    for system in \
        $(((i + 7) % 3 + 1)) \
        $(((i + 5) % 2 + 1)) \
    ; do
      system="system_$system"
      echo "    $system/"
      i=$(((i + 51) % 31 + 1))
      for version in \
        "$(((i % 5) + 1)).0.$((i))" \
        "$(((i % 5) + 1)).0.$((i + 1))" \
        "$(((i % 4) + 1)).1.$((i))" \
      ; do
        echo "      $version.tar.xz"
        moduledir="$namespace/$module/$system/$version"
        mkdir -p "$moduledir"
        cat > "$moduledir/$module.tf" <<EOF
output "namespace" { value = "$namespace" }
output "module" { value = "$module" }
output "system" { value = "$system" }
output "version" { value = "$version" }
EOF
        (
          cd "$(dirname "$moduledir")"
          tar \
            --owner=0 \
            --group=0 \
            --mtime=0 \
            "${verbose[@]}" \
            -cf "$version.tar" \
            "$version" \
          ;
          xz "$version.tar"
        )
        rm -r "$moduledir"
      done
    done
  done
done
echo DONE
