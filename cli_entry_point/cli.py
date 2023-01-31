<<<<<<< HEAD:cli-entry-point/entry.py
import ctypes
import sys
=======
"""
CLI entry point for repository analyzer.
"""

>>>>>>> main:cli_entry_point/cli.py
from typing import List
from repo_analyzer_interface import RepoAnalyzer


def run_install():
    # python
    print("install functionality not implemented.")


def run_build():
    # python
    print("build functionality not implemented.")


def run_test():
    # python
    print("test functionality not implmemented.")


def run_help():
    # python
    print("help functionality not implemented.")


def run_repo_list():
    # rust
    interface = RepoAnalyzer("test.dll")
    interface.display_repo_list()


def run_url(url: str):
    # rust
    print(url)
    interface = RepoAnalyzer()
    interface.print_score_from_url()


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