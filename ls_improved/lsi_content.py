from .config import Config

class LsiContentTransforms():
    def __init__(self, search_word, limit_file_num):
        """
        Content Transforms (for children).
        e.g. custom sort function, grep function.

        Parameters
        ----------
        search_word : String
            (command) -s, --search
        limit_file_num : Int
            (command) -n, --limit-file-num
        """
        # Set Config
        self.config = Config()

        # Set Auguments
        self.search_word = search_word
        self.limit_file_num = limit_file_num

    def _search(self, children, prev_status):
        """
        Search word from path and description.
        This like grep function.

        Parameters
        ----------
        children : List[children_d, children_f]
            children_d : List[items]
            children_f : List[items]
        prev_status : Int

        Returns
        -------
        status : Boolean
            0 == success
            1 == failed
        output_children : List[children_d, children_f]
            children_d : List[items]
            children_f : List[items]
        """
        search_word = self.search_word
        replace_word = self.config.tag['search'] + search_word + self.config.tag['search_end']
        tags = list(self.config.color.keys())
        tag_mapper = []
        inv_tag_mapper = []
        hash = '\\' + self.config.symbol*21
        if set(self.config.symbol)==set(search_word) or \
           search_word[-1]=='\\' or search_word[0]=='\\' or \
           '\\'+self.config.symbol in search_word or self.config.symbol+'\\' in search_word:
            status = 1
            output_children = [[], []]
            invalid_word = str(set(hash))
            print('sorry, only', invalid_word, 'is invalid search word.')
            return status, output_children

        for tag in tags:
            tag_mapper += [hash+'\\']
            inv_tag_mapper += [tag]
            hash += self.config.symbol
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

    def _limit_file_num(self, children, condition):
        """
        Alert number of files if it exceeded limit.

        Parameters
        ----------
        children : List[children_d, children_f]
            children_d : List[items]
            children_f : List[items]
        prev_status : Int

        Returns
        -------
        status : Boolean
            0 == number of files <= limit
            1 == number of files > limit, and 'yes'
            2 == number of files < limit, and 'no'
        output_children : List[children_d, children_f]
            children_d : List[items]
            children_f : List[items]
        """
        limit_file_num = self.limit_file_num
        num = len(children[0]+children[1])
        if num<=limit_file_num:
            status = 0
            return status, children
        res = input('too many items ({}). show these? [y-n] : '.format(num))
        if res.lower() in ['y', 'yes']:
            status = 1
            return status, children
        else:
            status = 2
            return status, [[], []]

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
        transforms += [self._limit_file_num] if self.limit_file_num!=0 else []
        for tr in transforms:
            prev_status, children = tr(children, prev_status)
        status = 0
        return status, children
