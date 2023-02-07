import os

def build():
    os.system("cd ../repo_analyzer/; cargo build; ls ; mv target/debug/librepo_analyzer.so ../cli_entry_point ; cd ../cli_entry_point ; ls")