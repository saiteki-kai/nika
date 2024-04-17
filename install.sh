#!/bin/sh

LOCAL_INSTALL_DIR=$HOME/.local/bin

if [ ! -d "$LOCAL_INSTALL_DIR" ]; then
    mkdir -p "$LOCAL_INSTALL_DIR"
fi

INSTALL_DIR=$LOCAL_INSTALL_DIR

cp matching/dist/extract_words $INSTALL_DIR

# Build project
cargo build --release

# Move binaries
mv target/release/nika $INSTALL_DIR
mv target/release/nika-updater $INSTALL_DIR

# Setup crontab to run the updater every 3 days
(crontab -l 2>/dev/null; echo "0 12 */3 * * $INSTALL_DIR/nika-updater") | crontab -

# Run the updater
$INSTALL_DIR/nika-updater
