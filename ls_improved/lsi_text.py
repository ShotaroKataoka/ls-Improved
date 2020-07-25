from config import Config

config = Config()
class Text():
    def __init__(self, text, start_color):
        self.text, self.style = self._from_tag_text(text, start_color)

    def _from_tag_text(self, text, start_color):
        '''
        Parameters:
            text : String
            start_color : String
                start color tag
        
        Returns:
            text_string : String
            text_style : List[style]
                style : Dict
                    style.keys: [tag, start_pos, end_pos]
        '''
        text = [{'tag':start_color, 'text':text}]
        for tag in set(config.tag.values())-set([';ss;', ';se;', ';dw;']):
            new_text_list = [[t] for t in text]
            for i, t in enumerate(text):
                splited_text = t['text'].split(tag)
                if len(splited_text)==1:
                    new_text = [t]
                elif len(splited_text)>1:
                    t['text'] = splited_text[0]
                    new_text = [t]
                    for st in splited_text[1:]:
                        new_text += [{'tag':tag, 'text':st}]
                new_text_list[i] = new_text
            text = []
            for t in new_text_list:
                text += t
        text_string = ''
        text_style = []
        text_count = 0
        for t in text:
            text_string += t['text']
            text_style += [{'tag': t['tag'], 'start_pos': text_count, 'end_pos': text_count+len(t['text'])}]
            text_count += len(t['text'])
        text_style += [{'tag': ';end;', 'start_pos': text_count, 'end_pos': text_count}]
        return text_string, text_style
    
    def insert_text(self, text, pos):
        self.text = self.text[:pos] + text + self.text[pos:]
        for i in range(len(self.style)):
            if self.style[i]['end_pos']>=pos:
                if self.style[i]['start_pos']>=pos:
                    self.style[i]['start_pos'] += len(text)
                self.style[i]['end_pos'] += len(text)

    def insert_style(self, color_tag, pos):
        new_style = []
        for i, style in enumerate(self.style):
            if pos > style['start_pos'] and style['end_pos'] >= pos:
                end_pos = style['end_pos']
                self.style[i]['end_pos'] = pos
                break
        self.style += [{'tag': color_tag, 'start_pos': pos, 'end_pos': end_pos}]

    
    def render(self):
        self.style.sort(key=lambda x: x['start_pos'])
        text = ''
        for tag in self.style:
            s = tag['start_pos']
            e = tag['end_pos']
            color = config.color[tag['tag']]
            text += color + self.text[s:e]
        print(text)
