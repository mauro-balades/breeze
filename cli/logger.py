from termcolor import colored

class logger:
    class prefix:
        info = colored("info", "green", attrs=["bold"])
        error = colored("error", "red", attrs=["bold"])

    def info(message: str) -> None:
        print(f"[{logger.prefix.info}]: {message}")

    def error(message: str) -> None:
        print(f"[{logger.prefix.error}]: {message}")
