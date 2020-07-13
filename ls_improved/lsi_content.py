from config import Config

class LsiContentTransforms():
    def __init__(self, search_word):
        # Set Config
        self.config = Config()

        # Set Auguments
        self.search_word = search_word

    def _search(self, children, prev_status):
        search_word = self.search_word
        replace_word = self.config.tag['search'] + search_word + self.config.tag['search_end']
        tags = list(self.config.color.keys())
        tag_mapper = []
        inv_tag_mapper = []
        hash = ' ;;;'
        for tag in tags:
            tag_mapper += [hash+' ']
            inv_tag_mapper += [tag]
            hash += ';'
        output_children = [[], []]
        for item in children[0]+children[1]:
            match_path = search_word in item['path']
            if 'description' in item.keys():
                description = item['description']
                for hash, tag in zip(tag_mapper, inv_tag_mapper):
                    description = description.replace(tag, hash)
                match_desc = search_word in description
            else:
                match_desc = False
            if match_path or match_desc:
                if match_path:
                    item['path'] = item['path'].replace(search_word, replace_word)
                if match_desc:
                    description = description.replace(search_word, replace_word)
                    for hash, tag in zip(tag_mapper, inv_tag_mapper):
                        description = description.replace(hash, tag)
                    item['description'] = description
                if item['type']=='Dir':
                    output_children[0] += [item]
                elif item['type']=='File':
                    output_children[1] += [item]
            else:
                pass
        status = 0
        return status, output_children

    def run(self, children, condition):
        """
        This apply content_transforms to all items.

        Parameters
        ----------
        item : Dict
        condition : Dict
        
        Returns
        -------
        status : Boolean
            0 == success
            1 == failed
        children : List[children_d, children_f]
            children_d : List[items]
            children_f : List[items]
        """
        prev_status = condition['status']
        transforms = []
        transforms += [self._search] if self.search_word!='' else []
        for tr in transforms:
            prev_status, children = tr(children, prev_status)
        status = 0
        return status, children
