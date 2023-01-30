import sys
from typing import List


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
    print("repo list functionality not implemented.")


def run_url(url: str):
    # rust
    print(url)
    print("url input functionality not implmemented")


def run(args: List[str]):
    if len(args) > 1:
        print("Incorrect arg count!")
        return
    elif len(args) == 0:
        # runs help command if there are no args
        run_help()

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
