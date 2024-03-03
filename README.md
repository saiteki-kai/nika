# NIKA

[![CI](https://github.com/saiteki-kai/nika/actions/workflows/ci.yml/badge.svg)](https://github.com/saiteki-kai/nika/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/saiteki-kai/nika/graph/badge.svg?token=ostDT5Ufwc)](https://codecov.io/gh/saiteki-kai/nika)
![GitHub License](https://img.shields.io/github/license/saiteki-kai/nika)

Nika is a CLI application for daily japanese learning.

## Features

Main Features:

- [x] Dictionary updater
- [ ] Daily words
- [ ] Daily kanji
- [ ] Mark word/kanji status (skipped, done, etc.)
- [ ] User preferences (e.g. number of daily words/kanjis)
- [ ] Progress tracking (e.g. streak, average words/kanjis per day)
- [x] Random words
- [ ] Random kanjis
- [ ] Dictionary search
- [ ] Examples
- [ ] Links to online dictionaries

Future Features:

- [ ] DBpedia definitions
- [ ] Morphological analysis of sensentes
- [ ] Daily grammar
- [ ] Favorites
- [ ] History
- [ ] SRS
- [ ] Similar words

## Requirements

This project has the following requirements:

- [Rust and Cargo](https://www.rust-lang.org/tools/install): Cargo is the package manager for Rust. It is needed to build and run the project.
- [Crontab](https://en.wikipedia.org/wiki/Cron): Cron is a time-based job scheduler in Unix-like operating systems. It is used in this project to maintain the dictionary updated.

## Installation

Clone the repo:

```bash
git clone https://github.com/saiteki-kai/nika
cd nika/
```

Run the installer:

```bash
chmod +x install.sh
./install.sh
```

## Usage

Run nika:

```bash
nika --help
```

Update the dictionary manually:

```bash
nika-updater
```

## License

The original source code of this project is licensed under the terms of the GPLv3 [license](LICENSE).

### JMDICT & KANJIDIC

This package uses the [JMdict/EDICT](https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project) and [KANJIDIC](https://www.edrdg.org/wiki/index.php/KANJIDIC_Project) dictionary files. These files are the property of the Electronic Dictionary Research and Development Group, and are used in conformance with the Group's [licence](https://www.edrdg.org/edrdg/licence.html).

The modified versions of these files are sourced from [jmdict-simplified](https://github.com/scriptin/jmdict-simplified).
