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
        self.symbol = ';'

        """ Set Visual Setting """
        self.tag = {
                'dir': self.symbol + 'dir' + self.symbol,
                'file': self.symbol + 'file' + self.symbol,
                'description': self.symbol + 'desc' + self.symbol,
                'search': self.symbol + 'ss' + self.symbol,
                'search_end': self.symbol + 'se' + self.symbol,
                'end': self.symbol + 'end' + self.symbol,
                'red': self.symbol + 'red' + self.symbol
                }
        self.color = {
                self.symbol + 'dir' + self.symbol : PyColor.UNDERLINE+PyColor.CYAN,
                self.symbol + 'file' + self.symbol : PyColor.WHITE,
                self.symbol + 'desc' + self.symbol: PyColor.YELLOW,
                self.symbol + 'ss' + self.symbol: PyColor.REVERCE,
                self.symbol + 'se' + self.symbol: PyColor.END,
                self.symbol + 'end' + self.symbol: PyColor.END,
                self.symbol + 'red' + self.symbol: PyColor.RED
                }
        self.indent = self.get_color('end')+' ── '


    def get_color(self, color):
        return self.color[self.tag[color.lower()]]

