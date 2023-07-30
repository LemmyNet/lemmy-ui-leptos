#!/usr/bin/env bash

# Copy old src to /tmp
mkdir .tmp
cp -R src .tmp/lemmysrc

# Run leptosfmt
leptosfmt -c .leptosfmt.toml src

# Diff the dirs
diff -r src .tmp/lemmysrc/src
rm -rf .tmp
