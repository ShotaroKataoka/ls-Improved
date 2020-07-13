import os
from glob import glob

from config import Config

class LsiItemLoader():
    def __init__(self):
        # Set Config
        self.config = Config()

    def _get_children(self, dir, show_all=False, get_only_directories=False, get_only_files=False):
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
        children_d = [p for p in pathes if os.path.isdir(p)] if not get_only_files else []
        children_f = [p for p in pathes if os.path.isfile(p)] if not get_only_directories else []
        if children_d+children_f == []:
            status = 1
            return status, [[],[]]

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
        base_path = path.split('/')[-1]
        if os.path.isdir(path):
            s, description = self._read_description(path)
            has_desc = True if description is not None else False
            item = {
                    'path': base_path, 
                    'path_length': len(base_path), 
                    'type': 'Dir',
                    'depth': 0
                    }
            if has_desc:
                item['description'] = description
            status = 0
        elif os.path.isfile(path):
            item = {
                    'path': base_path, 
                    'path_length': len(base_path), 
                    'type': 'File',
                    'depth': 0
                    }
            status = 1
        else:
            item = {}
            status = 2
        return status, item

    def get_items(self, dir, show_all=False, show_only_directories=False, show_only_files=False):
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
        status, children = self._get_children(dir, show_all=show_all, get_only_directories=show_only_directories, get_only_files=show_only_files)
        children_d, children_f = children
        children_d = [self._create_item(child)[1] for child in children_d]
        children_f = [self._create_item(child)[1] for child in children_f]
        children = [children_d, children_f]
        return status, children

