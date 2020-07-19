import os
import argparse

class Mkdiri():
    def __init__(self, dir, description, is_add_mode=False):
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

        self.desc_name = '.description.lsi'

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
        if not os.path.exists(dir_path):
            os.mkdir(dir_path)
        self._write_description(dir_path+self.desc_name, description)

    def _raise_file_exists_error(self, dir_path, description):
        """
        If file exists, it will be called.
        If not add_mode, Raise error.

        Parameters
        ----------
        dir_path : String
            Directory Path
        description : String
            description text
        """
        if self.is_add_mode:
            self._write_description(dir_path+self.desc_name, description)
        else:
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
    parser.add_argument('dir', type=str, metavar='DirectoryPath', help='Directory which you want to make.')
    parser.add_argument('description', type=str, metavar='STRING' ,nargs='?', default='', help='Description text (default:No description.)')
    parser.add_argument('-a', '--add', action='store_true', help='Add description if directory already exists. (default:False)')
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
