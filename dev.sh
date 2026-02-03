#!/bin/sh
set -e

# Install tailwindcss locally if node_modules doesn't exist
#if [ ! -d "node_modules" ]; then
#  npm install
#fi

#tailwindcss \
#  -i ./packages/ui/input.css \
#  -o ./packages/ui/assets/output.css \
#  --watch &

#tailwindcss \
#  -i ./packages/web/input.css \
#  -o ./packages/web/assets/output.css \
#  --watch &

exec dx serve --package web --addr 0.0.0.0