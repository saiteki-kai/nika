#!/bin/bash

echo "Getting latest release of 'en_core_web_sm' from https://github.com/explosion/spacy-models/releases ..."

RELEASE_URL=$(wget -q -O - https://api.github.com/repos/explosion/spacy-models/releases | jq -r '.[] | select(.name | contains ("en_core_web_sm")) | .assets[] | select(.content_type | contains ("x-tgz")) | .browser_download_url')

if [ -z "$RELEASE_URL" ]; then
    echo "Could not find 'en_core_web_sm'. Try to install it manually."
    exit 1
fi

echo "Downloading $RELEASE_URL ..."

wget $RELEASE_URL -P data/ >/dev/null 2>&1

if [ $? -ne 0 ]; then
    echo "Could not download 'en_core_web_sm'. Try to install it manually."
    exit 1
fi

echo "Extracting files..."

cd data/ && tar -xzf $(basename "$RELEASE_URL") && cd ..

VERSION=$(basename "$RELEASE_URL" | awk -F- '{print $NF}' | awk -F. '{print $1"."$2"."$3}')

DEST_DIR="en_core_web_sm"
SRC_DIR="./data/en_core_web_sm-$VERSION/en_core_web_sm/en_core_web_sm-$VERSION/"

if [ ! -d "$SRC_DIR" ]; then
    echo "$SRC_DIR does not exist. Check if it was downloaded correctly."
    exit 1
fi

echo "Building executable..."

pyinstaller --onefile ./extract_words.py --add-data="$SRC_DIR:$DEST_DIR"
