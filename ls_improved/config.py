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

        """ Set Visual Setting """
        self.tag = {
                'dir': ';dir;',
                'file': ';file;',
                'description': ';desc;',
                'search': ';ss;',
                'search_end': ';se;',
                'end': ';end;',
                'red': ';red;'
                }
        self.color = {
                ';dir;' : PyColor.UNDERLINE+PyColor.CYAN,
                ';file;' : PyColor.WHITE,
                ';desc;': PyColor.YELLOW,
                ';ss;': PyColor.REVERCE,
                ';se;': PyColor.END,
                ';end;': PyColor.END,
                ';red;': PyColor.RED
                }

        """ Set description_path """
        self.description_name = '.description.lsi'
        self.indent = self.get_color('end')+' ── '


    def get_color(self, color):
        return self.color[self.tag[color.lower()]]

