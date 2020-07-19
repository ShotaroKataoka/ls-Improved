# ls-Improved: descriptive ls-like-command working on Python.
[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)
## What is it?
**ls-Improved (lsi)** prints out a directory structure with its descriptions. 
I've been using it when tackling some seriese of experiments.(e.g. machine leaning experiments.)  

### Example
We have some directories like below.  
Sometimes we'd like to access best result of experiment, but it is not easy.  It is necessary to open all of directories or note experiments_summary.txt beforehand in order to do it.  
![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/topic/v0.2.7-README/ISSUE54/doc/images/ls_using.png)

Now we have the **ls-Improved(lsi)** command here.  It allow us to see list of directories with its descriptions simultaneously.  
![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/topic/v0.2.7-README/ISSUE54/doc/images/lsi_using.png)

## Requirements
python2.7とpython3.7で動作確認  
It is working on Python, so this command is not depend on OS.  

## Setup
### Install
#### pip
```
pip install ls-Improved
```

#### 手動
pip installがいやな人とかパスが変になる人は手動インストールをお試しください。  
`wget https://github.com/ShotaroKataoka/ls-Improved/archive/v0.2.2.beta0.manual.zip`  
(releaseの最新バージョンのmanual versionをダウンロード)  
ダウンロードしたzipファイルを `unzip` する。  
解凍されたディレクトリを好きな場所に移動して`~~~/bin/`のパスを通すか，`~~~/bin/`配下の２ファイルを`/usr/local/bin/`に移動するかしてパスを通す。  

### Uninstall
`pip uninstall ls-Improved`

### Upgrade
`pip install --upgrade ls-Improved`

## Usage
### mkdiri
`mkdiri 作成するディレクトリ ディレクトリに付加する説明文` : ディレクトリを作成し，説明文を作成  
`mkdiri 作成するディレクトリ` : ディレクトリを作成し，説明文を初期値で作成  
`mkdiri -a 既存ディレクトリ ディレクトリに付加する説明文` : 既存のディレクトリに説明文を上書き  

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
