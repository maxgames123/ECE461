import os

def install():
    os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh") # installs rustup