from .config import Config
from .lsi_text import Text


class LsiVisualTransforms():
    def __init__(self):
        """
        Visual Transforms (for item).
        e.g. set color, set indent.
        """
        # Set Config
        self.config = Config()

    def _add_indent_to_new_line(self, item, prev_status):
        """
        Visual transform for Description.
        Add indent to new line. \n -> \n____

        Parameters
        ----------
        item : Dict
        prev_status : Boolean

        Returns
        -------
        status : Boolean
        item : Dict
        """
        if 'description' not in item.keys():
            status = 1
            return status, item
        indent_length = 3 * (item['depth']+1)
        base_name = item['path']
        description = item['description'].text

        blank = '│' + ' '*int(indent_length + len(item['path']) + 3)
        description = description.split('\n')
        for i, desc in enumerate(description[::-1]):
            if not (set(' ') == set(desc) or set('') == set(desc)):
                break
        description = description[:len(description) - i]
        item['description'].text = '\n'.join(description)
        
        if len(description)>=2:
            insert_count = 0
            for desc in description[:-1]:
                insert_count += len(desc)+1
                item['description'].insert_text(blank, insert_count)
                item['description'].insert_style(';nl;', insert_count)
                item['description'].insert_style(';nle;', insert_count+len(blank))
                insert_count += len(blank)
        status = 0
        return status, item

    def _select_indent_head(self, item, place):
        """
        Select indent head ├,└

        Parameters
        ----------
        item : Dict
        place : Int
            0 if item is not last of children
            1 if item is last of children

        Returns
        -------
        head : String (├ or └)
        item : Dict
        """
        if place==0:
            return '├', item
        if place==1:
            if 'description' in item.keys():
                item['description'].text = item['description'].text.replace('│', ' ')
            return '└', item
        if place==2:
            return '', item


    def _concat_item(self, item, place):
        """
        Concatenate all texts.
        Output final string like below.
        'file name / description\n
                     new line description'

        Parameters
        ----------
        item : Dict
        prev_status : Boolean

        Returns
        -------
        status : Boolean
        output : String
        """
        head, item = self._select_indent_head(item, place)
        if 'description' in item.keys():
            description = item['description']
        else:
            description = Text(item['type'], ';w;')
        indent = head+' '*3*item['depth'] + self.config.indent
        if place==2:
            output = description.render()
        else:
            output = indent + item['path'].render() + ' / ' + description.render()
        status = 0
        return status, output

    def run(self, item, condition):
        """
        This apply visual_transforms to an item.

        Parameters
        ----------
        item : Dict
        condition : Dict
        
        Returns
        -------
        status : Boolean
            0 == success
            1 == failed
        output : String
            An visualized item.
            This will be printed to the terminal.
        """
        prev_status = condition['status']
        transforms = []
        if condition['is_last']!=2:
            transforms += [self._add_indent_to_new_line]
        for tr in transforms:
            prev_status, item = tr(item, prev_status)

        status, output = self._concat_item(item, condition['is_last'])
        return status, output
