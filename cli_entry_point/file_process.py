
def open_file(url_file: str):
    # Read file contents
    FILE = open(url_file, 'r')
    url_list = [line.rstrip('\n') for line in FILE]
    FILE.close()
    return url_list