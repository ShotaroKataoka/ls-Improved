import os
import argparse
from glob import glob


# Other settings
class pycolor:
    red = '\033[31m'
    green = '\033[32m'
    yellow = '\033[33m'
    BLUE = '\033[34m'
    PURPLE = '\033[35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'
    END = '\033[0m'
    BOLD = '\038[1m'
    UNDERLINE = '\033[4m'
    INVISIBLE = '\033[08m'
    REVERCE = '\033[07m'


class Lsi():
    def __init__(self, dir, is_all=False):
        self.dir = dir
        self.is_all = is_all

        self.desc_name = '.description.lsi'
        self.c_dir = pycolor.CYAN
        self.c_desc = pycolor.yellow
        self.c_end = pycolor.END
        self.c_under = pycolor.UNDERLINE

    # Raise Error
    def _assert_dir_existance(self, dir):
        if not os.path.isdir(dir):
            print('error: ' + dir[:-1] + ' is not a directory.')
            exit()

    # Get children
    def _get_children_of_dir(self, dir):
        pathes = []
        if self.is_all:
            pathes = glob(dir+'.*')
        pathes += glob(dir+'*')
        children_d = [p for p in pathes if os.path.isdir(p)]
        children_f = [p for p in pathes if os.path.isfile(p)]
        return children_d, children_f

    # load descriprion
    def _read_description(self, desc_path, dir_length):
        if os.path.exists(desc_path):
            with open(desc_path, 'r') as f:
                description = f.read().replace('\n', '\n       '+' '*dir_length)
        else:
            description = 'Dir'

        # description is empty
        if description == '':
            description = 'Dir'
        return description
    
    def _print_children_d(self, children_d):
        for dir in children_d:
            dir_name = dir.split('/')[-1]
            dir_length = len(dir_name)
            dir_name = self.c_dir + self.c_under + dir_name + self.c_end
            desc_path = dir +'/' + self.desc_name
            description = self._read_description(desc_path, dir_length)
            ## 最終行が空白のみの場合除去
            if set(description.split('\n')[-1])==set(' '):
                description = '\n'.join(description.split('\n')[:-1])
            ## descriptionが指定されているなら色を付ける
            description = self.c_desc + description + self.c_end if description != 'Dir' else description
            output = ' -- ' + dir_name + ' / ' + description
            print(output)

    def _print_children_f(self, children_f):
        for file in children_f:
            file_name = file.split('/')[-1]
            description = 'File'
            description = self.c_desc + description + self.c_end if description != 'File' else description
            output = ' -- ' + file_name +' / '+description
            print(output)

    def run(self):
        self._assert_dir_existance(self.dir)
        children_d, children_f = self._get_children_of_dir(self.dir)
        self._print_children_d(children_d)
        self._print_children_f(children_f)


def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="lsi ==lsImproved==")
    parser.add_argument('dir', type=str, nargs='?', default="./", metavar='DirectoryPath', help='directory where you want to look. (default: current directory)')
    parser.add_argument('-a','--all', action='store_true', help='show hidden files and directories. (default: Hidden)')
    args = parser.parse_args()

    # Get parser arguments
    dir = args.dir
    dir = dir+'/' if dir[-1] != '/' else dir
    is_all = args.all

    lsi = Lsi(dir, is_all=is_all)
    lsi.run()

if __name__ == '__main__':
    main()
