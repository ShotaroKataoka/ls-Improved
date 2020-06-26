import os
import argparse
from glob import glob


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
    BACK_LIGHT_YELLOW = '\033[230m'
    BACK_BLACK = '\033[40m'


class Lsi():
    def __init__(self, dir, is_all=False, is_only_directories=False, is_only_files=False, search_word=None):
        self.dir = dir
        self.is_all = is_all
        self.is_only_files = is_only_files
        self.is_only_directories = is_only_directories
        self.search_word = search_word

        self.desc_name = '.description.lsi'

        # Visual Settings
        self.c_dir = pycolor.CYAN
        self.c_desc = pycolor.YELLOW
        self.c_end = pycolor.END
        self.c_under = pycolor.UNDERLINE
        self.cb_search = pycolor.REVERCE
        self.cb_black = pycolor.BACK_BLACK
        self.normal_indent = self.c_end+' ── '

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
            # get directory name
            dir_name = dir.split('/')[-1]
            dir_length = len(dir_name)
            dir_name = self.c_dir + self.c_under + dir_name + self.c_end

            # get description
            desc_path = dir +'/' + self.desc_name
            description = self._read_description(desc_path, dir_length)

            # search (grep)
            if self.search_word is not None:
                dir_name, description, is_matched = self._search_word_from_1sentence(dir_name, description, self.search_word)
                if not is_matched:
                    continue

            ## 最終行が空白のみの場合除去
            if set(description.split('\n')[-1])==set(' '):
                description = '\n'.join(description.split('\n')[:-1])
            ## descriptionが指定されているなら色を付ける
            description = self.c_desc + description + self.c_end if description != 'Dir' else description
            output = self.normal_indent + dir_name + ' / ' + description
            print(output)

    def _print_children_f(self, children_f):
        for file in children_f:
            file_name = file.split('/')[-1]
            description = 'File'

            # search (grep)
            if self.search_word is not None:
                file_name, description, is_matched = self._search_word_from_1sentence(file_name, description, self.search_word)
                if not is_matched:
                    continue

            description = self.c_desc + description + self.c_end if description != 'File' else description
            output = self.normal_indent + file_name +' / '+description
            print(output)

    def _print_children(self, children_d, children_f, num_len):
        # ファイル数がnum_class以上のときに表示するか尋ねる
        def _confirm():
            res = input('too many items. show these? [y-n] : ')
            return res.lower() in ['y', 'yes'] 

        # -f -d で分岐
        if self.is_only_directories:
            if len(children_d) > num_len:
                if _confirm():
                    self._print_children_d(sorted(children_d))
            else:
                self._print_children_d(sorted(children_d))

        if self.is_only_files:
            if len(children_f) > num_len:
                if _confirm():
                    self._print_children_f(sorted(children_f))
            else:
                self._print_children_f(sorted(children_f))

        if not self.is_only_directories and not self.is_only_files:
            if (len(children_f) + len(children_d)) > num_len:
                if _confirm():
                    self._print_children_d(sorted(children_d))
                    self._print_children_f(sorted(children_f))
            else:
                self._print_children_d(sorted(children_d))
                self._print_children_f(sorted(children_f))

    def _search_word_from_1sentence(self, item_name, description, search_word):
        """
        Execute matching a 'word' with a 'sentence'.
        Then the sentence with 'matched word' the color of
        which is inversed is returned.
        If matched word does not exist, return None.

        Parameters
        ----------
        item_name : string
            directory or file name.
        description : string
            description string.
        search_word : string
            -s option's input.  e.g.) 'something'. 

        Returns
        -------
        item_name : string
            directory or file name.
        description : string
            description string. 'word'
        is_matched : boolean
            if word exists in item_name or description, True.
            else False.
        """
        is_matched_item = search_word in item_name
        is_matched_desc = search_word in description
        is_matched = is_matched_item or is_matched_desc
        replace_word = self.cb_search + self.search_word + self.c_end
        if is_matched_item:
            item_name = item_name.replace(search_word, replace_word+self.c_dir)
        if is_matched_desc:
            description = description.replace(search_word, replace_word+self.c_desc)
        return item_name, description, is_matched


    def run(self):
        self._assert_dir_existance(self.dir)
        children_d, children_f = self._get_children_of_dir(self.dir)
        self._print_children(children_d, children_f, 50)



def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="lsi ==lsImproved==")
    parser.add_argument('dir', type=str, nargs='?', default="./", metavar='DirectoryPath', help='directory where you want to look. (default: current directory)')
    parser.add_argument('-a','--all', action='store_true', help='show hidden files and directories. (default: Hidden)')
    parser.add_argument('-d','--only-directories', action='store_true', help='show only directories.')
    parser.add_argument('-f','--only-files', action='store_true', help='show only files.')
    parser.add_argument('-s','--search', default=None, help='search word inside of file names and descriptions')
    args = parser.parse_args()

    # Get parser arguments
    dir = args.dir
    dir = dir+'/' if dir[-1] != '/' else dir
    is_all = args.all
    is_only_directories = args.only_directories
    is_only_files = args.only_files
    search_word = args.search if args.search != '' else None

    lsi = Lsi(dir, is_all=is_all, is_only_directories=is_only_directories, is_only_files=is_only_files, search_word=search_word)
    lsi.run()

if __name__ == '__main__':
    main()
