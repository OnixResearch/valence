#!/usr/bin/env bash
set -Eeuo pipefail
BASE=/home/brittonr/mantle-cairn-finish
exec nix develop "$BASE/xrepo/cairn-source-review-main-20261020" --override-input octet 'git+ssh://git@github.com/OnixResearch/octet.git?rev=2f8213a88781d9d38bb81319d06a50805d48ccd3' --no-write-lock-file --command python3 "$BASE/bin/jobs/valence-receiver-policy-acceptance-20261020-v1.py"
