
from src.errors import ConfigurationError
from .cpp import build as cpp
from .c   import build as c

import importlib

native_langs = {
    "c": c,
    "cpp": cpp,
}

def build(config):
    lang = config["project"]["lang"]
    method = native_langs.get(lang, None)

    if method is None:

        try:
            imported_lang = importlib.import_module(f"breeze-{lang}")
            method = imported_lang.get("breeze_build", None)

            if method is None:
                raise ConfigurationError(f"Language support for {lang} does not have a 'breeze_build' function exported!")

        except ImportError:
            raise ConfigurationError(f"Language {lang} not supported! \n  note: try executing \"pip3 install breeze-{lang}\"")

    method(config)