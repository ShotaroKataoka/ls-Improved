import os


class PyColor:
    """ Standard IO Color Settings. """
    RED = '\033[1;31m'
    GREEN = '\033[1;32m'
    YELLOW = '\033[33m'
    BLUE = '\033[1;34m'
    PURPLE = '\033[1;35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'
    END = '\033[0m'
    BOLD = '\038[1m'
    UNDERLINE = '\033[4m'
    INVISIBLE = '\033[08m'
    REVERCE = '\033[07m'
    BACK_LIGHT_YELLOW = '\033[230m'
    BACK_BLACK = '\033[40m'
    LIGHT_CYAN = '\033[1;36m'


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
                'pwd': self.symbol + 'pwd' + self.symbol,
                'pwd_current': self.symbol + 'pwd_c' + self.symbol,
                'dir': self.symbol + 'dir' + self.symbol,
                'file': self.symbol + 'file' + self.symbol,
                'description': self.symbol + 'desc' + self.symbol,
                'search': self.symbol + 'ss' + self.symbol,
                'search_end': self.symbol + 'se' + self.symbol,
                'end': self.symbol + 'end' + self.symbol,
                'end_user': self.symbol + 'e' + self.symbol,
                'underline': self.symbol + '_' + self.symbol,
                'red': self.symbol + 'r' + self.symbol,
                'red2': self.symbol + 'red' + self.symbol,
                'green': self.symbol + 'g' + self.symbol,
                'green2': self.symbol + 'green' + self.symbol,
                'blue': self.symbol + 'b' + self.symbol,
                'blue2': self.symbol + 'blue' + self.symbol,
                'purple': self.symbol + 'p' + self.symbol,
                'purple2': self.symbol + 'purple' + self.symbol,
                'white': self.symbol + 'w' + self.symbol,
                'white2': self.symbol + 'white' + self.symbol,
                'description_white': self.symbol + 'dw' + self.symbol
                }
        self.color = {
                self.tag['pwd'] : PyColor.UNDERLINE+PyColor.CYAN,
                self.tag['pwd_current'] : PyColor.LIGHT_CYAN,
                self.tag['dir'] : PyColor.UNDERLINE+PyColor.CYAN,
                self.tag['file'] : PyColor.WHITE,
                self.tag['description'] : PyColor.YELLOW,
                self.tag['search'] : PyColor.REVERCE,
                self.tag['search_end'] : PyColor.END,
                self.tag['end'] : PyColor.END,
                self.tag['underline'] : PyColor.UNDERLINE,
                self.tag['red'] : PyColor.RED,
                self.tag['red2'] : PyColor.RED,
                self.tag['green'] : PyColor.GREEN,
                self.tag['green2'] : PyColor.GREEN,
                self.tag['blue'] : PyColor.BLUE,
                self.tag['blue2'] : PyColor.BLUE,
                self.tag['purple'] : PyColor.PURPLE,
                self.tag['purple2'] : PyColor.PURPLE,
                self.tag['white'] : PyColor.WHITE,
                self.tag['white2'] : PyColor.WHITE,
                self.tag['description_white'] : PyColor.WHITE
                }
        self.color[self.tag['end_user']] = PyColor.END+self.get_color('description')
        self.indent = self.get_color('end')+'── '


    def get_color(self, color):
        return self.color[self.tag[color.lower()]]

