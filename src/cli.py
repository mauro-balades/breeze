import argparse
import toml

from src.errors import CommandNotFound
from src.build import build as build_project
from src import helpers
from .logger import *

def get_arguments():
    args_parser = argparse.ArgumentParser()

    subparser = args_parser.add_subparsers(dest='command')
    build_arg = subparser.add_parser("build", help="build the current project")
    build_arg.add_argument("--config", help="specify configuration file", default="breeze.toml")

    return args_parser, args_parser.parse_args()

class CLI:

    def execute(self, command, argv):
        if command == "build":
            return CLI.build(argv)
        else:
            raise CommandNotFound(f"Command '{command}' not found!")

    def build(argv):

        logger.info(f"Opening configuration file ({argv.config})")
        config = toml.load(open(argv.config))
        helpers.check_config(config)

        logger.info(f"Building project for {config['project']['lang']}")

        folder = helpers.create_breeze_folder()
        config[".folder"] = folder

        build_project(config)

        print("")
        logger.success(f"Done building {config['project']['name']}! ✨")
