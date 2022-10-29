
from .cli import CLI, get_arguments
from .logger import *

def main():

    args_parser, args = get_arguments()

    if args.command is None:
        args_parser.print_help()
        return

    cli = CLI()

    try:
        cli.execute(args.command, args)
    except Exception as e:
        logger.error(str(e))
        exit(1)

