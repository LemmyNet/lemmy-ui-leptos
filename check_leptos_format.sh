# Copy old src to /tmp
cp -R src /tmp/lemmysrc

# Run leptosfmt
leptosfmt -c .leptosfmt.toml src

# Diff the dirs
diff src /tmp/lemmysrc
