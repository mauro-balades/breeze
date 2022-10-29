
from .cli import CLI, get_arguments

def main():

    args_parser, args = get_arguments()

    if args.command is None:
        args_parser.print_help()
        return

    cli = CLI()
    cli.execute(args.command, args)



