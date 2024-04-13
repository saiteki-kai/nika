# Utility for extracting context words from text

Preprocess text with [spacy](https://spacy.io/), extract context words, and save them to a file.

## Usage

```shell
./dist/extract_words -i input.json -o output.json
```

## Setup

Create a virtual environment

```shell
python3 -m venv venv
source venv/bin/activate
```

Install dependencies (SpaCy, PyInstaller)

```shell
pip install -r requirements.txt
```

## Installation

Run the `./install.sh` script to download the latest version of spacy model and build the executable.

## Manual Installation

Download spacy model manually

```shell
wget -P data https://github.com/explosion/spacy-models/releases/download/en_core_web_sm-3.7.1/en_core_web_sm-3.7.1.tar.gz
cd data/ && tar -xzf en_core_web_sm-3.7.1.tar.gz
```

Build the executable with pyinstaller

```shell
pyinstaller --onefile extract_words.py --add-data="./data/en_core_web_sm-3.7.1/en_core_web_sm/en_core_web_sm-3.7.1:en_core_web_sm"
```
