
from .cpp import build as cpp

native_langs = {
    "cpp": cpp
}

def build(config):
    method = native_langs[config["project"]["lang"]]
    method(config)