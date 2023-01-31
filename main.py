"""
The start of our adventure.
"""

import sys

if __name__ == "__main__":
    from cli_entry_point.cli import run
    run(sys.argv[1:])
