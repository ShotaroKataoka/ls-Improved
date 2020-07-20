from .config import Config

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
        description = item['description']

        blank = '\n'+';dw;│;se;' + ' '*int(indent_length + item['path_length'] + 3)
        description = description.split('\n')
        if len(description)>=2:
            if set(description[-1])==set(' ') or description[-1]=='':
                description = description[:-1]
        description = blank.join(description)
        status = 0
        item['description'] = description+';end;'
        return status, item

    def _add_color_to_path(self, item, prev_status):
        """
        Visual transform for Path.
        Add color to Path text.

        Parameters
        ----------
        item : Dict
        prev_status : Boolean

        Returns
        -------
        status : Boolean
        item : Dict
        """
        type = item['type']
        config = self.config
        if type=='Dir':
            item['path'] = config.get_color('dir') + item['path'] + config.get_color('end')
        elif type=='File':
            item['path'] = config.get_color('file') + item['path'] + config.get_color('end')
        status = 0
        return status, item

    def _tag2color(self, item, prev_status):
        """
        Visual transform for Description.
        change tag text to color code.

        Parameters
        ----------
        item : Dict
        prev_status : Boolean

        Returns
        -------
        status : Boolean
        item : Dict
        """
        config = self.config
        text = item['path']
        text = item['path'].replace(config.tag['search'], config.get_color('search'))
        text = text.replace(config.tag['search_end'], config.get_color('search_end')+config.get_color(item['type']))
        item['path'] = text
        if 'description' not in item.keys():
            status = 1
            return status, item

        description = [{'tag':config.tag['description'], 'text':item['description']}]
        for tag in set(config.tag.values())-set([';ss;', ';se;', ';dw;']):
            new_description = [[desc] for desc in description]
            for i, desc in enumerate(description):
                splited_text = desc['text'].split(tag)
                if len(splited_text)==1:
                    new_desc = [desc]
                elif len(splited_text)>1:
                    desc['text'] = splited_text[0]
                    new_desc = [desc]
                    for text in splited_text[1:]:
                        new_desc += [{'tag':tag, 'text':text}]
                new_description[i] = new_desc
            description = []
            for desc in new_description:
                description += desc
        output_description = ''
        for desc in description:
            text = desc['text'].replace(config.tag['search'], config.get_color('search'))
            text = text.replace(config.tag['description_white'], config.get_color('description_white'))
            text = text.replace(config.tag['search_end'], config.get_color('search_end')+config.color[desc['tag']])
            output_description += config.color[desc['tag']]
            output_description += text
        item['description'] = output_description
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
                item['description'] = item['description'].replace(self.config.get_color('description_white')+'│'+self.config.get_color('search_end'), self.config.get_color('search_end')+' ')
            return '└', item


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
            description = item['type']
        indent = head+'a'*3*item['depth'] + self.config.indent
        output = indent + item['path'] + ' / ' + description
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
        transforms += [self._add_indent_to_new_line]
        transforms += [self._tag2color]
        transforms += [self._add_color_to_path]
        for tr in transforms:
            prev_status, item = tr(item, prev_status)

        status, output = self._concat_item(item, condition['is_last'])
        return status, output
