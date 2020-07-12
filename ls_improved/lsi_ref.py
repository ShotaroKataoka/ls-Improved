import os
import argparse
from glob import glob

from .config import Config


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
        config = Config()

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
        desc_path = dir + config.description_name
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
            Dict.keys.default = ['path', 'type', 'depth']
            Dict.keys.optional = ['description', 'auth', 'children']
        """
        if os.path.isdir(path):
            s, description = self._read_description(path)
            has_desc = True if description is not None else False
            item = {
                    'path': path, 
                    'type': 'Dir'
                    'depth': 0
                    }
            if has_desc:
                item['description'] = description
            status = 0
        elif os.path.isfile(path):
            item = {
                    'path': path,
                    'type': 'File'
                    'depth': 0
                    }
            status = 1
        else:
            item = {}
            status = 2
        return status, item

    def create_children_items(self, dir, show_all=False):
        """
        
        """
        status, children = self._get_children(dir, show_all=show_all)
        children_d, children_f = children
        children_d = [self._create_item(child)[1] for child in children_d]
        children_f = [self._create_item(child)[1] for child in children_f]
        children = [children_d, children_f]
        return status, children

    def _visual_tr_new_line(self, desc, dir_length, depth=0):
        indent_length = 4 * (depth+1)
        blank = '\n'+' '*int(indent_length + dir_length + 3)
        desc = desc.split('\n')
        desc = blank.join(desc)
        status = 0
        return status, desc

    def _visual_tr_manager(self, item):
        base_name = item['path'].split('/')[-1]
        depth = item['depth']
        if 'description' in item.keys():
            description = item['description']
            description = self._visual_tr_new_line(desc,len(len(base_name)), depth)
        else:
            description = item['type']
        indent = ' '*4*depth + config.indent
        output = indent + base_name + ' / ' + description
        return output

    def make_visual(self, children, print_with_running=False):
        children_d = children[0]
        children_f = children[1]
        

    def run(self):
        """
        Management all functions.
        """
        dir = self.dir
        status, children = self.create_children_items(dir, show_all=self.show_all)
        
        

