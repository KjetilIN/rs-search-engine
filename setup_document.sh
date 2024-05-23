#!/bin/bash

# Enable error handling
set -e

echo "[INFO] Cloning the Kubernetes Website repository..."
# Clone the repository with depth 1
git clone --depth=1 https://github.com/kubernetes/website.git

echo "[INFO] Creating the root folder named pages..."
# Create a root folder named pages
mkdir -p pages

echo "[INFO] Copying content from website/content/en/ to pages/..."
# Copy content from website/content/en/ to pages/
cp -r website/content/en/* pages/ 

echo "[INFO] Removing the website folder..."
# Remove the website folder
rm -rf website 

echo "[INFO] Removing all non-html files from pages..."
# Remove all files in pages that are not HTML files
find pages -type f ! -name "*.html" -exec rm -f {} \; 

echo "[INFO] Cleaning up empty directories in pages..."
# Remove empty directories in pages
find pages -type d -empty -delete 

echo "[INFO] Process completed successfully."