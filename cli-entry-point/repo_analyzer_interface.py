import ctypes

# This file holds all of the logic used to interact with the rust library
UTF8_ENCODING = "utf-8"


class RepoAnalyzer:
    def __init__(self, dll_path: str = "repo_analyzer.dll", encoding: str = UTF8_ENCODING):
        self.lib = ctypes.WinDLL(dll_path)
        self.encoding = encoding

    def display_repo_list(self):
        func = self.lib.display_repo_list
        func()

    def print_score_from_url(self):
        func = self.lib.print_score_from_url
        func()

# ignore this function, but this logic is needed later so do not delete it
def example_rust_lib_func_call():
    strl = ctypes.WinDLL("repo_analyzer.dll")
    # r_lib = ctypes.cdll.LoadLibrary("repo_analyzer.dll")
    # print(r_lib.get_string().decode('utf-8'))

    get_string = strl.get_string
    get_string.argtypes = [ctypes.c_char_p]
    get_string.restype = ctypes.c_char_p
    input_str = "Hello".encode(UTF8_ENCODING)
    result_ptr = get_string(input_str)
    result = ctypes.string_at(result_ptr)
    print(result.decode(UTF8_ENCODING))
