#!/bin/bash

# Directory to save HTML files
output_dir="pages"
mkdir -p "$output_dir"

# File containing the list of URLs
url_file="./cache/urls.txt"

# Function to sanitize filenames
sanitize_filename() {
    local filename="$1"
    # Replace spaces with underscores
    filename="${filename// /_}"
    # Remove special characters
    filename=$(echo "$filename" | tr -cd '[:alnum:]_-')
    echo "$filename"
}

# Read the URLs from the file
while IFS=";" read -r url title; do
    # Sanitize the title to create a valid filename
    sanitized_title=$(sanitize_filename "$title")

    # Download the file
    echo "Downloading $url"
    wget "$url" -O "$output_dir/$sanitized_title.html"
done < "$url_file"

echo "Done."
