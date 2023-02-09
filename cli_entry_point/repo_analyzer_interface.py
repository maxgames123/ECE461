import ctypes

# This file holds all of the logic used to interact with the rust library
UTF8_ENCODING = "utf-8"


class RepoAnalyzer:
    def __init__(self, so_path: str = "../repo_analyzer/target/debug/librepo_analyzer.so", encoding: str = UTF8_ENCODING):
        try:
            self.lib = ctypes.CDLL(so_path)
        except OSError:
            print("Unable to load " + so_path + ". Make sure it is located in the correct path.")
            exit(0)
        self.encoding = encoding

    def display_repo_list(self):
        func = self.lib.display_repo_list
        func.restype = ctypes.c_char_p
        func()

    def print_score_from_url(self, url: str):
        func = self.lib.print_score_from_url
        func(url)


# ignore this function, but this logic is needed later so do not delete it
def example_rust_lib_func_call():
    strl = ctypes.CDLL("librepo_analyzer.so")
    # r_lib = ctypes.cdll.LoadLibrary("librepo_analyzer.so")
    # print(r_lib.get_string().decode('utf-8'))

    get_string = strl.get_string
    get_string.argtypes = [ctypes.c_char_p]
    get_string.restype = ctypes.c_char_p
    input_str = "Hello".encode(UTF8_ENCODING)
    result_ptr = get_string(input_str)
    result = ctypes.string_at(result_ptr)
    print(result.decode(UTF8_ENCODING))
