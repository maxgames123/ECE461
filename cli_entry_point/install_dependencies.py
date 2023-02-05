import os

def install():
    os.system("curl --proto '=https' -sSf https://sh.rustup.rs | sh") # installs rustup
    os.system("cargo install cargo-edit")
    os.system("cd ../repo_analyzer; cargo add reqwest")