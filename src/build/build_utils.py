
import os

def is_newer(input: str, output: str) -> bool:
    return os.path.getmtime(input) > os.path.getmtime(output)

def can_compile(input: str, output: str) -> bool:
    if not os.path.exists(output):
        return True

    return is_newer(input, output)
