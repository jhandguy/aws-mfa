#!/bin/bash

set -euo pipefail

file="Cargo.toml"
new_version="version = \"${1}\""

version_pattern="[0-9]*\.[0-9]*\.[0-9]*"
old_version="version = \"$(grep -x "version = \"${version_pattern}\"" "${file}" | grep -o "${version_pattern}")\""

toml="$(cat "${file}")"
echo "${toml//${old_version}/${new_version}}" > "${file}"

cargo update
