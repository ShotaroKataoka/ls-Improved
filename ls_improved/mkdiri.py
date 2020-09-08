import os
import argparse

from .lsi_text import Text
from .lsi_itemloader import LsiItemLoader
from .config import Config


class Mkdiri():
    def __init__(self, dir, description, is_add_mode=False, is_file_mode=False):
        """
        Make directory with description file.

        Parameters
        ----------
        dir : String
            Directory Path
        description : String
            Description text
        is_add_mode : Boolean (Optional)
            overwrite existing description if True.
        """
        self.dir = dir
        self.description = description
        self.is_add_mode = is_add_mode
        self.is_file_mode = is_file_mode
        config = Config()

        self.desc_name = config.description_name
        self.file_desc_name = config.file_description_name

    def _write_description(self, desc_path, description):
        """
        Make description file.

        Parameters
        ----------
        desc_path : String
            description file path
        description : String
            description text
        """
        description = description.replace('\\n', '\n')
        with open(desc_path, 'w') as f:
            f.write(description)

    def _write_file_description(self, file_path, description_dict):
        """
        Make file description.

        Parameters
        ----------
        file_path : String
            file description path
        description_dict : Dict
            description Dict(LsiItemLoader.read_file_description)
        """
        keys_list = sorted(list(description_dict.keys()))
        description = ''
        for keys in keys_list:
            desc = description_dict[keys]
            for i in range(len(desc)):
                if desc[len(desc)-1-i]!='\n':
                    break
            desc = desc[:len(desc)-i]
            description += '\\/'+keys+'\n'+desc+'\n\n'
        with open(file_path, 'w') as f:
            f.write(description)

    def _mkdiri(self, dir_path, description):
        """
        Make directory and make description file.

        Parameters
        ----------
        dir_path : String
            Directory Path
        description : String
            description text
        """
        dir_path = dir_path+'/' if dir_path[-1] != '/' else dir_path
        if os.path.exists(dir_path):
            if self.is_add_mode:
                self._write_description(dir_path+self.desc_name, description)
            else:
                self._raise_file_exists_error(self.dir_path)
        else:
            os.mkdir(dir_path)
            self._write_description(dir_path+self.desc_name, description)

    def _mkfilei(self, file_path, description):
        """
        Make file description.

        Parameters
        ----------
        file_path : String
            File Path
        description : String
            description text
        """
        split_path = file_path.split('/')
        if len(split_path)==1:
            base_path = './'
        else:
            base_path = '/'.join(split_path[:-1]) + '/'
        base_name = split_path[-1]
        _, description_dict = LsiItemLoader().read_file_description(base_path)

        description = description.replace('\\n', '\n')
        description_dict[base_name] = description
        self._write_file_description(base_path+self.file_desc_name, description_dict)

    def _raise_file_exists_error(self, dir_path):
        """
        If file exists, it will be called.
        If not add_mode, Raise error.

        Parameters
        ----------
        dir_path : String
            Directory Path
        """
        print('error: "' + dir_path[:-1] + '" already exists.')
        print('if you want to add a description, use "-a" option please.')
        exit()

    def _raise_path_not_found_error(self):
        """
        If path not found, raise error.
        """
        print('error: invalid path.')
        exit()

    def _raise_permission_denied_error(self):
        """
        If not have permission, raise error.
        """
        print('error: Permission denied.')
        exit()

    def run(self):
        """
        Run all functions.
        """
        try:
            if self.is_file_mode:
                self._mkfilei(self.dir, self.description)
            else:
                self._mkdiri(self.dir, self.description)
        except OSError as e:
            e = str(e)
            if 'File exists' in e:
                self._raise_file_exists_error(self.dir, self.description)
            if 'No such file or directory' in e:
                self._raise_path_not_found_error()
            if 'Permission denied' in e:
                self._raise_permission_denied_error()
            print(e)

def main():
    # Parser setting
    parser = argparse.ArgumentParser(description="MKDIRI - MKDIR Improved")
    parser.add_argument('dir', type=str, metavar='DirectoryPath',nargs='?', default='', help='Directory which you want to make.')
    parser.add_argument('description', type=str, metavar='STRING' ,nargs='?', default='', help='Description text (default:No description.)')
    parser.add_argument('-a', '--add', action='store_true', help='Add description if directory already exists. (default:False)')
    parser.add_argument('-f', '--file', action='store_true', help='make file description. (default:False)')
    parser.add_argument('--ANSI-MODE', action='store_true', help='SET ANSI-ESCAPE-SEQUENCE-MODE.')
    parser.add_argument('--LSI-MODE', action='store_true', help='SET LSI-ESCAPE-SEQUENCE-MODE.')
    parser.add_argument('-e', '--SHOW-MODE', action='store_true', help='SET LSI-ESCAPE-MODE.')
    args = parser.parse_args()
    config = Config()

    # ESCAPE SEQUENCE MODE
    ANSI_ESCAPE_SEQUENCE_MODE = args.ANSI_MODE
    LSI_ESCAPE_SEQUENCE_MODE = args.LSI_MODE
    SHOW_ESCAPE_SEQUENCE_MODE = args.SHOW_MODE

    if ANSI_ESCAPE_SEQUENCE_MODE and LSI_ESCAPE_SEQUENCE_MODE:
        print('error: invalid auguments.')
        exit()
    
    home = os.environ['HOME']
    lsirc = 'ESCAPE SEQUENCE = LSI'
    if os.path.exists(home+'/.lsirc'):
        with open(home+'/.lsirc', 'r') as f:
            lsirc = f.read()
    try:
        escape_sequence = lsirc.split('=')[1].replace(' ', '')
        if escape_sequence not in ['LSI', 'ANSI']:
            escape_sequence = 'LSI'
    except:
        escape_sequence = 'LSI'

    if ANSI_ESCAPE_SEQUENCE_MODE or LSI_ESCAPE_SEQUENCE_MODE:
        if ANSI_ESCAPE_SEQUENCE_MODE:
            escape_sequence = 'ANSI'
        if LSI_ESCAPE_SEQUENCE_MODE:
            escape_sequence = 'LSI'
        with open(home+'/.lsirc', 'w') as f:
            f.write('ESCAPE SEQUENCE = '+ escape_sequence)
        print('success: set', escape_sequence, 'mode and write it to ~/.lsirc')

    if SHOW_ESCAPE_SEQUENCE_MODE:
        print('ESCAPE SEQUENCE MODE = ' + escape_sequence)


    # Get parser arguments
    dir = args.dir
    if dir == '':
        exit()
    description = args.description
    if escape_sequence=='ANSI':
        description = Text(description, ';desc;')
        description = description.render()
        description = description[5:].split(config.get_color('end'))
        while description[-1]=='':
            description = description[:-1]
        description = config.get_color('end').join(description)
    is_add_mode = args.add
    is_file_mode = args.file

    mkdiri = Mkdiri(dir, description, is_add_mode, is_file_mode)
    mkdiri.run()

if __name__ == '__main__':
    main()
