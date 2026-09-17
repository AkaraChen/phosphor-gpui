#!/bin/sh
# Refresh assets from phosphor-icons/core at the pin in phosphor.json.
set -eu
cd "$(dirname "$0")/.."
VERSION=$(python3 -c 'import json; print(json.load(open("phosphor.json"))["version"])')
REV=$(python3 -c 'import json; print(json.load(open("phosphor.json"))["rev"])')
WEIGHTS=$(python3 -c 'import json; print(" ".join(json.load(open("phosphor.json"))["weights"]))')
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
SPARSE=""
for weight in $WEIGHTS; do
	SPARSE="$SPARSE assets/$weight"
done
git clone --depth 1 --branch "v$VERSION" --filter=blob:none --sparse \
	https://github.com/phosphor-icons/core.git "$TMP/core"
git -C "$TMP/core" sparse-checkout set $SPARSE
HEAD=$(git -C "$TMP/core" rev-parse HEAD)
if [ "$HEAD" != "$REV" ]; then
	echo "phosphor.json rev $REV does not match v$VERSION ($HEAD)" >&2
	exit 1
fi
for weight in $WEIGHTS; do
	rm -rf "assets/$weight"
	mkdir -p "assets/$weight"
	cp -R "$TMP/core/assets/$weight/." "assets/$weight/"
done
COUNT=$(find assets/regular -name '*.svg' | wc -l | tr -d ' ')
python3 - "$COUNT" <<'PY'
import json, sys
count = int(sys.argv[1])
path = "phosphor.json"
data = json.load(open(path))
data["icon_count"] = count
json.dump(data, open(path, "w"), indent="\t")
open(path, "a").write("\n")
print(f"synced {count} icons in each weight")
PY
