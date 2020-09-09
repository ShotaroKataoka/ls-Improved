import os
import re


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
    BOLD = '\033[1m'
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
        self.file_description_name = '.file_description.lsi'

        """ Set Visual Setting """
        self.tag = {
                'pwd': ';pwd;',
                'pwd_current': ';pwd_c;',
                'dir': ';dir;',
                'file': ';file;',
                'description': ';desc;',
                'search': ';ss;',
                'search_end': ';se;',
                'end': ';end;',
                'newline': ';nl;',
                'newlineend': ';nle;',
                'end_user': ';e;',
                'underline': ';_;',
                'red': ';r;',
                'yellow': ';y;',
                'green': ';g;',
                'blue': ';b;',
                'purple': ';p;',
                'white': ';w;',
                'description_white': ';dw;'
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
                self.tag['newline'] : PyColor.END,
                self.tag['newlineend'] : PyColor.END,
                self.tag['underline'] : PyColor.UNDERLINE,
                self.tag['red'] : PyColor.RED,
                self.tag['yellow'] : PyColor.YELLOW,
                self.tag['green'] : PyColor.GREEN,
                self.tag['blue'] : PyColor.BLUE,
                self.tag['purple'] : PyColor.PURPLE,
                self.tag['white'] : PyColor.WHITE,
                self.tag['description_white'] : PyColor.WHITE
                }
        self.color[self.tag['end_user']] = PyColor.END+self.get_color('description')

        ANSI_ESCAPE_SEQUENCE_PATTERN = r'\\033\[(?:[0-9]+;)*[0-9]+m'
        self.ANSI_ESCAPE_SEQUENCE_PATTERN = re.compile(ANSI_ESCAPE_SEQUENCE_PATTERN)

        self.indent = self.get_color('end')+'── '


    def get_color(self, color):
        return self.color[self.tag[color.lower()]]

    def get_color_from_tag(self, tag):
        if '\\033' in tag:
            return tag.replace('\\033', '\033')
        return self.color[tag]
