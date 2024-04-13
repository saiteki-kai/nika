import argparse
import json
import re
import string
from pathlib import Path

import spacy
from tqdm import tqdm

BUNDLE_DIR = Path(__file__).parent.absolute()
MODEL_PATH = BUNDLE_DIR / "en_core_web_sm"

EXCLUDED_WORDS = {"ie", "eg", "s", "t", ""}
EG_RE = re.compile("e\.g\..*?\)")


def clean(text: str) -> str:
    """Remove punctuation from a string."""
    return text.translate(str.maketrans("", "", string.punctuation))


def get_context_words(doc: spacy.tokens.doc.Doc) -> set[str]:
    """Get the context words from a spacy doc."""
    words = {clean(t.lemma_.lower()) for t in doc if not t.is_punct}
    words = list(words - EXCLUDED_WORDS)

    return words


def preprocess(entries: dict[str, list[str]]) -> dict[str, set[str]]:
    """Preprocess the entries.

    - Remove punctuation
    - Remove e.g. examples in the brackets
    - Convert to lower case
    - Keep only ascii characters
    - Lemmatize the words
    - Exclude certain words (e.g. 'ie', 'eg', 's', 't') and empty strings
    """
    nlp = spacy.load(MODEL_PATH, disable=["ner", "parser"])

    context = []

    for _, senses in entries.items():
        c = ". ".join([gloss for sense in senses for gloss in sense])

        c = re.sub(EG_RE, ")", c)
        c = c.encode("ascii", "ignore").decode()

        context.append(c)

    return {
        word_id: get_context_words(doc)
        for word_id, doc in tqdm(
            zip(list(entries.keys()), nlp.pipe(context, n_process=-1), strict=True),
            total=len(context),
        )
    }


def cli():
    """CLI entry point."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i", "--input", type=Path, help="Input file path for JSON entries."
    )
    parser.add_argument(
        "-o", "--output", type=Path, help="Output file path for JSON processed entries."
    )
    args = parser.parse_args()

    with args.input.open("r") as f:
        entries = json.loads(f.read())
        entries = preprocess(entries)

    with args.output.open("w") as f:
        f.write(json.dumps(entries))


if __name__ == "__main__":
    cli()
