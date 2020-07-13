from config import Config

class LsiVisualTransforms():
    def __init__(self):
        # Set Config
        self.config = Config()

    def _add_indent_to_new_line(self, item, prev_status):
        """
        Visual transform of Description.
        Add indent to new line. \n -> \n____

        Parameters
        ----------
        description : String
        dir_length : Int
            Length of directory name.
        depth : Int (Optional)
            Depth of directory. This control indent depth.
        """
        if 'description' not in item.keys():
            item['description'] = item['type']
            status = 1
            return status, item
        indent_length = 4 * (item['depth']+1)
        base_name = item['path']
        description = item['description']

        blank = '\n'+' '*int(indent_length + item['path_length'] + 3)
        description = description.split('\n')
        if len(description)>=2:
            if set(description[-1])==set(' ') or description[-1]=='':
                description = description[:-1]
        description = blank.join(description)
        status = 0
        item['description'] = description
        return status, item

    def _add_color_to_path(self, item, prev_status):
        type = item['type']
        config = self.config
        if type=='Dir':
            item['path'] = config.get_color('dir') + item['path'] + config.get_color('end')
        elif type=='File':
            item['path'] = config.get_color('file') + item['path'] + config.get_color('end')
        status = 0
        return status, item

    def _tag2color(self, item, prev_status):
        config = self.config
        text = item['path']
        text = item['path'].replace(config.tag['search'], config.get_color('search'))
        text = text.replace(config.tag['search_end'], config.get_color('search_end')+config.get_color(item['type']))
        item['path'] = text
        if 'description' not in item.keys():
            status = 1
            return status, item

        description = [{'tag':config.tag['description'], 'text':item['description']}]
        for tag in set(config.tag.values())-set([';ss;', ';se;', ';end;']):
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
            text = text.replace(config.tag['search_end'], config.get_color('search_end')+config.color[desc['tag']])
            output_description += config.color[desc['tag']]
            output_description += text
        item['description'] = output_description
        status = 0
        return status, item

    def _concat_item(self, item, prev_status):
        indent = ' '*4*item['depth'] + self.config.indent
        output = indent + item['path'] + ' / ' + item['description']
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
        transforms += [self._tag2color]
        transforms += [self._add_indent_to_new_line]
        transforms += [self._add_color_to_path]
        for tr in transforms:
            prev_status, item = tr(item, prev_status)

        status, output = self._concat_item(item, prev_status)
        return status, output
