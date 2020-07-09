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
        
        
        
        """ Set CommandLine Arguments """
        self.dir = dir
        self.show_all = show_all
        self.show_only_files = show_only_files
        self.show_only_directories = show_only_directories
        self.limit_file_num = limit_file_num
        self.show_file_num = show_file_num
        self.search_word = search_word
        
        """ Set Config """
        config = Config()

    def 


