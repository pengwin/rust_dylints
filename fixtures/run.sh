#!/usr/bin/env bash
# Lint the fixture crates with each library and compare the emitted lint names with
# fixtures/expected/<library>.txt: `bad` must trigger exactly that set, `good` and `composed` nothing.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(dirname "$here")"

emitted() {
    local library="$1" fixture="$2"
    { cargo dylint --path "$root/lints/$library" --manifest-path "$here/$fixture/Cargo.toml" -- --all-targets --message-format=json 2>/dev/null || true; } |
        python3 -c '
import json, sys
prefixes = ("layout_", "bevy_", "testing_")
names = set()
for line in sys.stdin:
    try:
        code = (json.loads(line).get("message") or {}).get("code") or {}
    except ValueError:
        continue
    if str(code.get("code", "")).startswith(prefixes):
        names.add(code["code"])
print("\n".join(sorted(names)))'
}

status=0
for expected_file in "$here"/expected/*.txt; do
    library="$(basename "$expected_file" .txt)"
    bad="$(emitted "$library" bad)"
    if [[ "$bad" != "$(cat "$expected_file")" ]]; then
        echo "$library: bad fixture lint set differs" >&2
        diff "$expected_file" <(echo "$bad") >&2 || true
        status=1
    fi
    for fixture in good composed; do
        silent="$(emitted "$library" "$fixture")"
        if [[ -n "$silent" ]]; then
            echo "$library: $fixture fixture triggered lints:" >&2
            echo "$silent" >&2
            status=1
        fi
    done
done
[[ $status -eq 0 ]] && echo "fixtures ok"
exit $status
