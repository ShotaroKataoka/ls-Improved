import os
import argparse


# Other settings
class pycolor:
    RED = '\033[31m'
    GREEN = '\033[32m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    PURPLE = '\033[35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'
    END = '\033[0m'
    BOLD = '\038[1m'
    UNDERLINE = '\033[4m'
    INVISIBLE = '\033[08m'
    REVERCE = '\033[07m'

class Mkdiri():
    def __init__(self, dir, description, is_add_mode=False):
        self.dir = dir
        self.description = description
        self.is_add_mode = is_add_mode

        self.desc_name = '.description.lsi'

    def _write_description(self, desc_path, description):
        with open(desc_path, 'w') as f:
            f.write(description)

    def _mkdiri(self, dir_path, description):
        os.mkdir(dir_path)
        self._write_description(dir_path+self.desc_name, description)

    def _raise_file_exists_error(self, dir_path, description):
        if self.is_add_mode:
            self._write_description(dir_path+self.desc_name, description)
        else:
            print('error: "' + dir_path[:-1] + '" already exists.')
            print('if you want to add a description, use "-a" option please.')
            exit()

    def _raise_path_not_found_error(self):
        print('error: invalid path.')

    def run(self):
        try:
            self._mkdiri(self.dir, self.description)
        except OSError as e:
            e = str(e)
            if 'File exists' in e:
                self._raise_file_exists_error(self.dir, self.description)
            if 'No such file or directory' in e:
                self._raise_path_not_found_error()

def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="mkdiri ==mkdirImproved==")
    parser.add_argument('dir', type=str, default="./", metavar='DirectoryPath', help='directory where you want to look. (default: current directory)')
    parser.add_argument('description', type=str, nargs='?', default='', help='description message (default:No description.)')
    parser.add_argument('-a','--add', action='store_true', help='add description if directory already exists. (default:False)')
    args = parser.parse_args()

    # Get parser arguments
    dir = args.dir
    dir = dir+'/' if dir[-1] != '/' else dir
    description = args.description
    is_add_mode = args.add

    mkdiri = Mkdiri(dir, description, is_add_mode)
    mkdiri.run()

if __name__ == '__main__':
    main()
