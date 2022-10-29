from termcolor import colored

class logger:
    class prefix:
        info = colored("info", "cyan", attrs=["bold"])
        error = colored("error", "red", attrs=["bold"])
        success = colored("success", "green", attrs=["bold"])

    def info(message: str) -> None:
        print(f"[{logger.prefix.info}]: {message}")


    def success(message: str) -> None:
        print(f"[{logger.prefix.success}]: {message}")

    def error(message: str) -> None:
        print(f"[{logger.prefix.error}]: {message}")
