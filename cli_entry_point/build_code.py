import os

def build():
    os.system("cd ../repo_analyzer; cargo build")
    print("Build functionality not finished yet!")

    # Fix the line of code below!
    # os.system("mv /target/debug/FILENAME_HERE!.dll ../cli_entry_point")