
import os
from src.errors import ConfigurationError

BREEZE_FOLDER = ".breeze"

def assert_dict(cfg, field, name = None):
    if name is None:
        name = field

    value = cfg.get(field, None)
    if value is None or value == "":
        raise ConfigurationError(f"Field '{name}' required in config file")

def create_breeze_folder():
    if not os.path.exists(BREEZE_FOLDER):
        os.mkdir(BREEZE_FOLDER)

    return BREEZE_FOLDER

def check_config(config: dict):
    assert_dict(config, "project")

    assert_dict(config["project"], "name", "project.name")
    assert_dict(config["project"], "lang", "project.lang")

    folder = config.get(".folder", None)
    if folder is not None:
        raise ConfigurationError("Field '.folder' can't exist at top level of config file")