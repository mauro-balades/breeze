import argparse

from cli.errors import CommandNotFound

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
        print("")
