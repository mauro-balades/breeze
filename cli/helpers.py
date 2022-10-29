
from cli.errors import ConfigurationError

def check_config(config: dict):
    def assert_dict(cfg, field, name = None):
        if name is None:
            name = field

        value = cfg.get(field, None)
        if value is None or value == "":
            raise ConfigurationError(f"Field '{name}' required in config file")

    assert_dict(config, "project")

    assert_dict(config["project"], "name", "project.name")
    assert_dict(config["project"], "lang", "project.lang")

    if (config["project"]["lang"] != "c" and
        config["project"]["lang"] != "cpp"):
        raise ConfigurationError(f"Language {config['project']['lang']} not supported!")
