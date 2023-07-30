#!/usr/bin/env bash
set -e

# Copy old src to /tmp
cp -R src /tmp/

# Run leptosfmt
leptosfmt -c .leptosfmt.toml src

# Diff the dirs
diff -r src /tmp/src
