# ls-Improved: descriptive ls-like-command working on Python.
[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)
## What is it?
**ls-Improved (lsi)** prints out a directory structure with its descriptions. 
I've been using it when tackling some seriese of experiments.(e.g. machine leaning experiments.)  

### Example
We have some directories like below.  
Sometimes we'd like to access a best result of experiment, but it is not easy.  It is necessary to open all of directories or note `experiments_summary.txt` beforehand in order to do it.  

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/topic/v0.2.7-README/ISSUE54/doc/images/ls_using.png)

Now we have the **ls-Improved(lsi)** command here.  It allows us to see list of directories with its descriptions simultaneously.  

![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/topic/v0.2.7-README/ISSUE54/doc/images/lsi_using.png)

## Requirements
developed on Python2.7 and Python3.7 (maybe ≧Python2.7 is ok.)  
It is working on Python, so this command dose not depend on OS.  

## Install
### pip install
```
# PyPI
pip install ls-Improved
```

### manual install
For person who do not like PyPI, manual install version exists.  

1. download latest manual version from github release.  
`wget https://github.com/ShotaroKataoka/ls-Improved/archive/v0.3.0.beta0.manual.zip`  
2. unzip downloaded zip file.  
3. set PATH environment to `bin/` directory.  
(set PATH to unziped `bin/` directory or place `bin/lsi` and `bin/mkdiri` to `/usr/local/bin/` )  

## Usage
### How works
`lsi` read `.description.lsi` files which are in each directories.  
`.description.lsi` is simple text file.  It is made by `mkdiri` command.  

**In brief:**
- `mkdiri` make a directory with the `.description.lsi` text file.
- `lsi` print out directory structure with `.description.lsi` content.

### mkdiri
`mkdiri` make a directory with the `.description.lsi`.
- `mkdiri DIRECTORY 'DESCRIPTION'` : make `DIRECTORY` and write `DESCRIPTION` into `.description.lsi`  
- `mkdiri DIRECTORY` : make `DIRECTORY` and create empty `.description.lsi`  
- `mkdiri -a DIRECTORY DESCRIPTION` : overwrite or create `.description.lsi` in existing directory.  

**tips:**  
Add decoration to description :  
- `\n` : make new line
- `;r;` `;g;` `;b;` `;w;` `;p;` : add color to text
- `;_;` : add underline to text
- `;e;` : end decoration

**Example**  
`mkdiri test 'This is ;p;test ;_;directory;e;.\nDecoration is useful!'`  

![mkdiri_decoration](https://github.com/ShotaroKataoka/ls-Improved/blob/topic/v0.2.7-README/ISSUE54/doc/images/mkdiri_decoration.png)  

### lsi
`lsi` : カレントディレクトリ内のファイルとディレクトリを表示  
`lsi path-to-directory` : パス内のファイルとディレクトリを表示  
`lsi -a` : 隠れファイル・ディレクトリも表示  
`lsi -f` : ファイルのみを表示
`lsi -d` : ディレクトリのみを表示
`lsi -s 'search-word'` : `search_word`でファイル名・説明文内を検索

### 仕組み
`mkdiri` はディレクトリ作成と同時に `.description.lsi` というテキストファイルを作成します。  
`lsi` はディレクトリ内の `.description.lsi` というテキストファイルを読み取って表示します。  
`.description.lsi` を直接編集することで説明文を編集することもできます。このとき，複数行の説明文を作成することも可能です。  
