#!/bin/sh

echo "Starting pre-flight checks..."

if [ ! -x "/app/sitemap" ]; then
    echo "CRITICAL ERROR: /app/sitemap binary is missing or not executable."
    echo "Container startup aborted."
    exit 1
fi

if [ ! -d "/app/blog" ]; then
    echo "CRITICAL ERROR: /app/blog directory is missing."
    echo "Container startup aborted."
    exit 1
fi

if [ ! -d "/app/public" ]; then
    echo "CRITICAL ERROR: /app/public directory is missing."
    echo "Container startup aborted."
    exit 1
fi

echo "Pre-flight checks passed. Generating sitemap..."

/app/sitemap

echo "Sitemap generated! Starting the web server..."

/app/website
