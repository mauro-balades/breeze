from termcolor import colored


class _logger:
    
    _BREEZE_CAN_DEBUG = False

    class prefix:
        info = colored("info", "cyan", attrs=["bold"])
        error = colored("error", "red", attrs=["bold"])
        success = colored("success", "green", attrs=["bold"])
        verbose = colored("debug", "magenta", attrs=["bold"])

    def can_debug(self):
        self._BREEZE_CAN_DEBUG = True

    def info(self, message: str) -> None:
        print(f"[{logger.prefix.info}]: {message}")

    def verbose(self, message: str) -> None:
        if self._BREEZE_CAN_DEBUG:
            print(f"[{logger.prefix.verbose}]: {message}")

    def success(self, message: str) -> None:
        print(f"[{logger.prefix.success}]: {message}")

    def error(self, message: str) -> None:
        print(f"[{logger.prefix.error}]: {message}")

logger = _logger()