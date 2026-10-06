#!/usr/bin/env bash
# End-to-end check that every generated solution template actually compiles.
#
# `cargo test` covers the Rust logic; this covers the *content* of the templates,
# which no unit test can assert (a template that does not compile is still a
# valid string). Run it after touching any *_TEMPLATE constant.
#
#   ./scripts/check-templates.sh
set -uo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src="$root/src/file_generator.rs"
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT

fail=0

# Extract a template's literal text straight from the Rust source, so this
# checks what actually ships rather than a copy that can drift.
emit() {
  python3 - "$src" "$out/$1" "$2" <<'PY'
import re, sys
src, dest, name = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(src).read()
m = re.search(r'pub const %s: &str = r#"(.*?)"#;' % name, text, re.S)
if not m:
    sys.exit("could not find template %s" % name)
open(dest, 'w').write(m.group(1))
PY
}

check() {
  local label="$1"; shift
  if "$@" >"$out/$label.log" 2>&1; then
    echo "  ok    $label"
  else
    echo "  FAIL  $label"
    sed 's/^/          /' "$out/$label.log" | head -20
    fail=1
  fi
}

echo "Template compile check:"

emit solution.cpp CPP_TEMPLATE
emit solution.py  PYTHON_TEMPLATE
emit solution.rs  RUST_TEMPLATE
emit solution.java JAVA_TEMPLATE

if command -v g++ >/dev/null; then
  check "cpp"  g++ -std=c++20 -O2 -Wall -Werror -o "$out/solution" "$out/solution.cpp"
else
  echo "  skip  cpp (no g++)"
fi

if command -v rustc >/dev/null; then
  check "rust" rustc --edition 2021 -o "$out/rs.out" "$out/solution.rs"
else
  echo "  skip  rust (no rustc)"
fi

if command -v javac >/dev/null; then
  # javac requires the public class to live in a file named after it, so the
  # template's class must be package-private to survive being written as
  # <problem>.java. Compile under that name to prove it.
  mkdir -p "$out/java"
  cp "$out/solution.java" "$out/java/Example_Problem.java"
  check "java" javac -d "$out/jout" "$out/java/Example_Problem.java"
else
  echo "  skip  java (no javac)"
fi

if command -v python3 >/dev/null; then
  check "python" python3 -c "import ast,sys;ast.parse(open(sys.argv[1]).read())" "$out/solution.py"
else
  echo "  skip  python (no python3)"
fi

if [ "$fail" -ne 0 ]; then
  echo
  echo "FAILED: at least one template does not compile."
  exit 1
fi

echo
echo "All templates compile."
