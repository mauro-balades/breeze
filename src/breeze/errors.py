
class CommandNotFound(Exception):
    """ Raised when a command not found by the CLI """

class ConfigurationError(Exception):
    """ Raised when there is something wrong in the config file """

class UnknownOutputType(Exception):
    """ Raised when output type is not supported """

