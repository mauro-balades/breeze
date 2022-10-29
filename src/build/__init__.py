
from src.errors import ConfigurationError
from .cpp import build as cpp

native_langs = {
    "cpp": cpp
}

def build(config):
    method = native_langs.get(config["project"]["lang"], None)

    if method is None:
        raise ConfigurationError(f"Language {config['project']['lang']} not supported!")

    method(config)