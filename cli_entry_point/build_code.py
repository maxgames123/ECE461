import os

def build:
    os.system("cd ../repo_analyzer")
    os.system("cargo build")
    os.system("mv /target/debug/filename.dll ../cli_entry_point")