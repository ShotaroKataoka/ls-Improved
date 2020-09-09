import os
from glob import glob

from .config import Config
from .lsi_text import Text

class LsiItemLoader():
    def __init__(self):
        """
        Item Loader

        Return 
        ------
        children : List[children_d, children_f]  
            children_d : List[items]
                item : Dict
                    dict.keys(default) = ['path', 'type', 'depth']
                    dict.keys(optional) = ['description', 'auth', 'children']
        """
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
            2 == permission denied
            3 == error
        description : String
            description string of directory.
        """
        dir = dir+'/' if dir[-1]!='/' else dir
        desc_path = dir + self.config.description_name
        try:
            with open(desc_path, 'r') as f:
                description = f.read()
            if description == '':
                description = None 
            status = 0
        except Exception as e:
            e = str(e)
            if 'no such file or directory' in e:
                description = None
                status = 1
            elif 'Permission denied' in e:
                description = ';w;Dir ' + self.config.get_color('red') + '(Permission denied)'
                status = 2 
            else:
                description = None
                status = 3
        return status, description

    def read_file_description(self, dir):
        """
        read .file_description.lsi

        Parameters
        ----------
        dir : String
            Directory Path

        Returns
        -------
        status : Int
            0 == success
            1 == description file not exists
        description : Dict
            key: file_name
            value: description String
        """
        dir = dir+'/' if dir[-1]!='/' else dir
        desc_path = dir + self.config.file_description_name
        try:
            with open(desc_path, 'r') as f:
                description = f.read()
            _, description = self._interpret_file_description(description)
            status = 0
        except:
            description = {}
            status = 1
        return status, description
    
    def _interpret_file_description(self, description):
        """
        interpret .file_description.lsi raw string (string -> dictionary)

        Parameters
        ----------
        description : String

        Returns
        -------
        status : Int
            0 == success
            1 == description file is empty
            2 == description content is invalid
        desc_dict : Dict
            key: file_name
            value: description String
        """
        if description=='':
            status = 1
            return status, {}
        description = description.split('\n')
        if len(description)<2:
            status = 2
            return status, {}

        file_name = '\\/'
        desc_dict = {}
        for desc in description:
            if len(desc)>=2:
                if desc[:2]=='\\/':
                    file_name = desc[2:]
                    continue
            try:
                desc_dict[file_name] += '\n'+desc
            except:
                desc_dict[file_name] = desc
        status = 0
        return status, desc_dict
        


    def _create_item(self, path, file_description={}):
        """
        Create directory or file dictionary.

        Parameters
        ----------
        path : String
            directory or file path.
        file_description : Dict (default: {})
            output of self.read_file_description
        
        Returns
        -------
        status : Int
            0 == path is dir
            1 == path is file
            2 == error
        item : Dict
            dict.keys(default) = ['path', 'type', 'depth']
            dict.keys(optional) = ['description', 'auth', 'children']
        """


        base_path = path.split('/')[-1]
        item = {
                'depth': 0
                }
        if os.path.isdir(path):
            s, description = self._read_description(path)
            has_desc = True if description is not None else False
            if has_desc:
                description = Text(description, ';desc;')
                item['description'] = description
            item['path'] = Text(base_path, ';dir;') 
            item['type'] = 'Dir'
            status = 0
        elif os.path.isfile(path):
            if base_path in file_description.keys():
                item['description'] = Text(file_description[base_path], ';desc;')
            item['path'] = Text(base_path, ';file;')
            item['type'] = 'File'
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
        top_item : Dict
            keys : path, children_d, children_f
                path : Strin (path of top directory)
                children_d : List[items]
                children_f : List[items]
        """
        if not os.path.isdir(dir):
            print('error: cannot open directory \''+dir+'\': No such directory.')
            exit()
        if not os.access(dir, os.R_OK):
            print('error: cannot open directory \''+dir+'\': Permission denied.')
            exit()
        status, children = self._get_children(dir, show_all=show_all, get_only_directories=show_only_directories, get_only_files=show_only_files)
        status, file_description = self.read_file_description(dir)
        children_d, children_f = children
        children_d = [self._create_item(child)[1] for child in children_d]
        children_f = [self._create_item(child, file_description=file_description)[1] for child in children_f]
        top_item = {
                'path': dir,
                'children_d': children_d,
                'children_f': children_f
                }
        return status, top_item

