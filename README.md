# ls-Improved: descriptive ls-command working on Python.
[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)
## What is it?
**ls-Improved (lsi)** prints out a directory structure with its descriptions. 
I've been using it when tackling some seriese of experiments.(e.g. machine leaning experiments.)  

### Example
We have some directories like below.
```
experiment_00/ experiment_01/ experiment_02/ experiment_03/ experiment_04/ experiment_05/
```
Sometimes we'd like to access best result of experiment, but it is not easy.  If we will do it, we will have to open all of directories or note experiments_summary.txt beforehand and read it.  
Now we have the **ls-Improved(lsi)** command here.  It allow us to see list of directories with its descriptions.  
```
experiment_00 / score=0.85, lr=1e-6, batch_size=16
experiment_01 / score=0.90, lr=1e-3, batch_size=16
experiment_02 / score=0.88, lr=1e-6, batch_size=32
experiment_03 / score=0.80, lr=1e-3, batch_size=32
experiment_04 / score=0.95, lr=1e-6, batch_size=16, with BatchNorm
                best validation result.
experiment_05 / score=0.93, lr=1e-6, batch_size=32, with BatchNorm
```

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
