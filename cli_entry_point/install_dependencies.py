import os

def install():
    os.system("curl --proto '=https' -sSf https://sh.rustup.rs | sh") # installs rustup
    os.system("cargo install cargo-edit")
    os.system("cd ../repo_analyzer; cargo add reqwest ; cargo add serde ; cargo add serde_json ; cargo add substring ; cargo add base64 ; cargo add async-recursion ; cargo add log ; cargo add env_logger")
    print("10 dependencies installed...")