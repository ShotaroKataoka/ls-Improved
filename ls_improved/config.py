import os


class PyColor:
    """ Standard IO Color Settings. """
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


class Config():
    def __init__(self):
        """
        Set shared config between lsi and mkdiri.
        Read '~/.lsirc' and adapt the settings.
        """

        """ Read .lsirc """
        lsirc = []
        if os.path.exists('~/.lsirc'):
            with open('~/.lsirc') as f:
                lsirc = f.readlines()

        """ Set description_path """
        self.description_name = '.description.lsi'
        self.indent = ' ── '

        """ Set Visual Setting """
        self.c_dir = PyColor.CYAN
        self.c_desc = PyColor.YELLOW
        self.c_under = PyColor.UNDERLINE
        self.c_search = PyColor.REVERCE
        self.c_back_black = PyColor.BACK_BLACK
        self.c_end = PyColor.END
        self.normal_indent = self.c_end+' ── '

