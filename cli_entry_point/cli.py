import ctypes
import sys

"""
CLI entry point for repository analyzer.
"""

from typing import List
from repo_analyzer_interface import RepoAnalyzer
from build_code import build
from install_dependencies import install


def run_install():
    # python
    install()


def run_build():
    # python
    build()


def run_test():
    # python
    print("test functionality not implmemented.")


def run_help():
    # python
    print("help functionality not implemented.")


def run_repo_list():
    # rust
    interface = RepoAnalyzer()
    r = interface.display_repo_list()
    print(r)


def run_url(filename: str):
    # rust
    print(filename)
    interface = RepoAnalyzer()
    interface.print_score_from_url(filename.encode(encoding = 'UTF-8'))


def run(args: List[str]):
    if len(args) > 1:
        print("Incorrect arg count!")
        return
    elif len(args) == 0:
        # runs help command if there are no args
        run_help()
        return

    arg = args[0]

    if arg == "install":
        run_install()
    elif arg == "build":
        run_build()
    elif arg == "test":
        run_test()
    elif arg == "help":
        run_help()
    elif arg == "rl":
        run_repo_list()
    else:
        run_url(arg)


if __name__ == "__main__":
    run(sys.argv[1:])
