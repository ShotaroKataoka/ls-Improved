import os
import argparse
from glob import glob

from config import Config

class Lsi():
    def __init__(
            self,
            dir,
            show_all=False,
            show_only_directories=False,
            show_only_files=False,
            show_file_num=False,
            limit_file_num=50,
            search_word=None
            ):
        """
        Constructor
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
        """
        
        # Set CommandLine Arguments
        self.dir = dir
        self.show_all = show_all
        self.show_only_files = show_only_files
        self.show_only_directories = show_only_directories
        self.limit_file_num = limit_file_num
        self.show_file_num = show_file_num
        self.search_word = search_word
        
        # Set Config
        self.config = Config()

    def _get_children(self, dir, show_all=False):
        """
        Get children files and directories of the directory.

        Parameters
        ----------
        dir : String
            Directory Path
        show_all : Boolean (Optional)
            True -> Get all files and directories.

        Returns
        -------
        status : Int
            0 == success
            1 == error
        children : List [children_d, children_f]
            children_d : List[String]
                children (directories)
            children_f : List[String]
                children (files)
        """

        # Get children
        pathes = glob(dir+'*')
        if show_all:
            pathes += glob(dir+'.*')
        children_d = [p for p in pathes if os.path.isdir(p)]
        children_f = [p for p in pathes if os.path.isfile(p)]

        # Prepare output
        children = [sorted(children_d), sorted(children_f)]
        status = 0
        return status, children

    def _read_description(self, dir):
        """
        Get .description.lsi of dir.

        Parameters
        ----------
        dir : String
            Directory Path

        Returns
        -------
        status : Int
            0 == success
            1 == description file not exists
            2 == error
        description : String
            description string of directory.
        """
        dir = dir+'/' if dir[-1]!='/' else dir
        desc_path = dir + self.config.description_name
        if os.path.exists(desc_path):
            with open(desc_path, 'r') as f:
                description = f.read()
            if description == '':
                description = None 
            status = 0
        else:
            description = None
            status = 1
        return status, description

    def _create_item(self, path):
        """
        Create directory or file dictionary.

        Parameters
        ----------
        path : String
            directory or file path.
        
        Returns
        -------
        status : Int
            0 == path is dir
            1 == path is file
            2 == error
        item : Dict
            Dict.keys(default) = ['path', 'type', 'depth']
            Dict.keys(optional) = ['description', 'auth', 'children']
        """
        if os.path.isdir(path):
            s, description = self._read_description(path)
            has_desc = True if description is not None else False
            item = {
                    'path': path.split('/')[-1], 
                    'type': 'Dir',
                    'depth': 0
                    }
            if has_desc:
                item['description'] = description
            status = 0
        elif os.path.isfile(path):
            item = {
                    'path': path.split('/')[-1],
                    'type': 'File',
                    'depth': 0
                    }
            status = 1
        else:
            item = {}
            status = 2
        return status, item

    def create_children_items(self, dir, show_all=False):
        """
        Repeat self._create_item() along all children of 'dir'.

        Parameters
        ----------
        dir : String
            Directory Path
        show_all : Boolean (Optional)
            Whether show hidden files or not.
        
        Return
        ------
        status : Int
            0 == success
            1 == error
        children : List[children_d, children_f]
            children_d : List[items of directories]
            children_f : List[items of files]
        """
        status, children = self._get_children(dir, show_all=show_all)
        children_d, children_f = children
        children_d = [self._create_item(child)[1] for child in children_d]
        children_f = [self._create_item(child)[1] for child in children_f]
        children = [children_d, children_f]
        return status, children

    def _visual_tr_add_indent_to_new_line(self, item):
        """
        Visual transform of Description.
        Add indent to new line. \n -> \n____

        Parameters
        ----------
        description : String
        dir_length : Int
            Length of directory name.
        depth : Int (Optional)
            Depth of directory. This control indent depth.
        """
        if 'description' not in item.keys():
            item['description'] = item['type']
            status = 1
            return status, item
        indent_length = 4 * (item['depth']+1)
        base_name = item['path']
        description = item['description']

        blank = '\n'+' '*int(indent_length + len(base_name) + 3)
        description = description.split('\n')
        if len(description)>=2:
            if set(description[-1])==set(' ') or description[-1]=='':
                description = description[:-1]
        description = blank.join(description)
        status = 0
        item['description'] = description
        return status, item

    def _visual_tr_add_base_color_to_description(self, item, prev_status):
        if prev_status==1:
            status = 1
            return status, item
        type = item['type']
        config = self.config
        if type=='Dir':
            item['description'] = config.c_desc + item['description'] + config.c_end
        elif type=='File':
            pass
        status = 0
        return status, item

    def _visual_tr_add_color_to_path(self, item):
        type = item['type']
        config = self.config
        if type=='Dir':
            item['path'] = config.c_dir + item['path'] + config.c_end
        elif type=='File':
            pass
        status = 0
        return status, item

    def _visual_tr_manager(self, item):
        """
        This apply visual_transforms to an item.

        Parameters
        ----------
        item : Dict
        
        Returns
        -------
        status : Boolean
            0 == success
            1 == failed
        output : String
            An visualized item.
            This will be printed to the terminal.
        """
        s, item = self._visual_tr_add_indent_to_new_line(item)
        s, item = self._visual_tr_add_base_color_to_description(item, s)
        s, item = self._visual_tr_add_color_to_path(item)

        indent = ' '*4*item['depth'] + self.config.indent
        output = indent + item['path'] + ' / ' + item['description']
        status = 0
        return status, output

    def print_items(self, children, print_with_running=False):
        """
        Repeat self._visual_tr_manager() along directories and files on this level.
        Then (or while), Print these.
        
        Parameters
        ----------
        children : List[children_d, children_f]
        print_with_running : Boolean (Optional)
            True -> Get by Print
            False -> Get and Print

        Return
        ------
        status : Boolean
            0 == success
            1 == failed
        """
        children_d = children[0]
        children_f = children[1]
        for item in children_d+children_f:
            s, output = self._visual_tr_manager(item)
            print(output)
        status = 0
        return status


    def run(self):
        """
        Management all functions.
        """
        dir = self.dir
        status, children = self.create_children_items(dir, show_all=self.show_all)
        self.print_items(children)
        
        
def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="lsi ==lsImproved==")
    parser.add_argument('dir', type=str, nargs='?', default="./", metavar='DirectoryPath', help='directory where you want to look. (default: current directory)')
    parser.add_argument('-a','--all', action='store_true', help='show hidden files and directories. (default: Hidden)')
    parser.add_argument('-d','--only-directories', action='store_true', help='show only directories.')
    parser.add_argument('-f','--only-files', action='store_true', help='show only files.')
    parser.add_argument('-s','--search', default=None, help='search word inside of file names and descriptions')
    parser.add_argument('-l','--show-file-num', action='store_true', help='show files num of directory')
    parser.add_argument('-n', '--limit-file-num', type=int, default=50, help='set threshold for opening directory by many files')
    args = parser.parse_args()

    # Get parser arguments
    dir = args.dir
    dir = dir+'/' if dir[-1] != '/' else dir
    show_all = args.all
    show_only_directories = args.only_directories
    show_only_files = args.only_files
    show_file_num = args.show_file_num
    limit_file_num = args.limit_file_num
    search_word = args.search if args.search != '' else None

    lsi = Lsi(
            dir,
            show_all=show_all, 
            show_only_directories=show_only_directories, 
            show_only_files=show_only_files, 
            show_file_num=show_file_num,
            limit_file_num=limit_file_num,
            search_word=search_word
            )

    lsi.run()

if __name__ == '__main__':
    main()
