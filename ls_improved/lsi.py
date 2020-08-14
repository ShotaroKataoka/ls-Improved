from __future__ import unicode_literals
import os
import sys
import argparse
from glob import glob

from .config import Config
from .lsi_itemloader import LsiItemLoader
from .lsi_content import LsiContentTransforms
from .lsi_visual import LsiVisualTransforms

version = sys.version_info
if version.major<=2 or (version.major==3 and version.minor<3):
    reload(sys)
    sys.setdefaultencoding('utf-8')

class Lsi():
    def __init__(
            self,
            dir,
            show_all=False,
            show_only_directories=False,
            show_only_files=False,
            show_file_num=False,
            limit_file_num=50,
            search_word=''
            ):
        """
        lsi Manager.
        Set all optional command line arguments here.
        
        Parameters
        ----------
        dir : String
            Directory Path
        show_all : Boolean (Optional)
            (command) -a, --all
            True -> Show all files and directories.
        show_only_directories : Boolean (Optional)
            (command) -d, --only-directories
            True -> Do not show files.
        show_only_files : Boolean (Optional)
            (command) -f, --only-files
            True -> Do not show directories.
        show_file_num : Boolean (Optional)
            (command) -l, --show-file-num
            True -> Show number of children files.
        limit_file_num : Int (Optional)
            (command) -n, --limit-file-num
            Set confirm condition (num of children files).
        search_word : String (Optional)
            (command) -s, --search
        """
        
        # Set CommandLine Arguments
        self.dir = dir
        self.show_all = show_all
        self.show_only_files = show_only_files
        self.show_only_directories = show_only_directories
        self.limit_file_num = limit_file_num
        self.show_file_num = show_file_num
        
        # Set Lsi Modules
        self.config = Config()
        self.item_loader = LsiItemLoader()
        self.content_transforms = LsiContentTransforms(
                search_word=search_word,
                limit_file_num=limit_file_num
                )
        self.visual_transforms = LsiVisualTransforms()

    def print_items(self, top_path, children, condition):
        """
        Repeat self._visual_tr_manager() along directories and files on this level.
        Then (or while), Print these.
        
        Parameters
        ----------
        top_path : String
        children : List[children_d, children_f]
        condition : Dict

        Return
        ------
        status : Boolean
            0 == success
            1 == failed
        """
        children = children[0]+children[1]
        if len(children)>0 and top_path!='./':
            top_path = os.path.abspath(top_path)
            above_path = self.config.get_color('pwd') + '/'.join(top_path.split('/')[:-1])+'/'
            base_name = self.config.get_color('pwd_current')+top_path.split('/')[-1]+'/'
            print(above_path+base_name+self.config.get_color('end'))
        for i, item in enumerate(children):
            if i+1==len(children):
                condition['is_last'] = 1
            else:
                condition['is_last'] = 0
            s, output = self.visual_transforms.run(item, condition)
            print(output)
        status = 0
        return status

    def run(self):
        """
        Management all functions.
        """
        status, top_item = self.item_loader.get_items(
                self.dir, 
                show_all=self.show_all,
                show_only_directories=self.show_only_directories,
                show_only_files=self.show_only_files
                )
        top_path = top_item['path']
        children = [top_item['children_d'], top_item['children_f']]

        condition = {
                'status': 0,
                }
        status, children = self.content_transforms.run(
                children,
                condition
                )

        condition = {
                'status': 0
                }
        status = self.print_items(
                top_path,
                children,
                condition
                )
        
        
def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="LSI - LS Improved")
    parser.add_argument('dir', type=str, nargs='?', default="./", metavar='DirectoryPath', help='Directory where you want to look. (default: current directory)')
    parser.add_argument('-a','--all', action='store_true', help='Show hidden files and directories. (default: Hidden)')
    parser.add_argument('-D','--only-directories', action='store_true', help='Do not show files.')
    parser.add_argument('-F','--only-files', action='store_true', help='Do not show directories.')
    parser.add_argument('-s','--search', default='', metavar='STRING', help='Search word inside of file names and descriptions')
    # parser.add_argument('-f','--show-file-num', action='store_true', help='show files num of directory')
    parser.add_argument('-n', '--limit-file-num', type=int, metavar='INT' ,default=0, help='set threshold for opening directory by many files')
    args = parser.parse_args()

    # Get parser arguments
    dir = args.dir
    dir = dir+'/' if dir[-1] != '/' else dir
    show_all = args.all
    show_only_directories = args.only_directories
    show_only_files = args.only_files
    # show_file_num = args.show_file_num
    limit_file_num = args.limit_file_num
    search_word = args.search

    lsi = Lsi(
            dir,
            show_all=show_all, 
            show_only_directories=show_only_directories, 
            show_only_files=show_only_files, 
            # show_file_num=show_file_num,
            limit_file_num=limit_file_num,
            search_word=search_word
            )

    lsi.run()

if __name__ == '__main__':
    main()
