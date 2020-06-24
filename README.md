# ls-Improved
## Description
ls-Improved (lsi)は大量のディレクトリ群から目当てのディレクトリを見つけることを補助するためのコマンドです。  
```
experiment_00
experiment_01
experiment_02
experiment_03
experiment_04
experiment_05
.......
```
このような機械的なディレクトリ群から必要なディレクトリを見つけることは容易ではありません。  
従来はテキストファイルや外部ツールを使ってこれらのディレクトリを管理する必要がありましたが，lsiではコマンド１つで下記のように一覧性良くディレクトリの説明文を表示することができます。  
```
experiment_00 / score=0.85, lr=1e-6, batch_size=16
experiment_01 / score=0.90, lr=1e-3, batch_size=16
experiment_02 / score=0.88, lr=1e-6, batch_size=32
experiment_03 / score=0.80, lr=1e-3, batch_size=32
experiment_04 / score=0.95, lr=1e-6, batch_size=16, with BatchNorm
                best validation result.
experiment_05 / score=0.93, lr=1e-6, batch_size=32, with BatchNorm
......
```

## Requirements
python2.7とpython3.7で動作確認  

## Setup
### Install
```
pip install https://github.com/ShotaroKataoka/ls-Improved/archive/v0.2.1.beta.zip
pip install --upgrade ls-Improved
```
pip2だとPATHが通らない可能性があります。  
その場合， `$HOME/.local/bin/` にいたりするので， `export PATH=$PATH:$HOME/.local/bin/` を `.bashrc` に追記するなどしてパスを通してください。  

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
`lsi ディレクトリのパス` : パス内のファイルとディレクトリを表示  
`lsi -a` : 隠れファイル・ディレクトリも表示  

### 仕組み
`mkdiri` はディレクトリ作成と同時に `.description.lsi` というテキストファイルを作成します。  
`lsi` はディレクトリ内の `.description.lsi` というテキストファイルを読み取って表示します。  
`.description.lsi` を直接編集することで説明文を編集することもできます。このとき，複数行の説明文を作成することも可能です。  
