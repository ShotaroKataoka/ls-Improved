# ls-Improved : 説明文つきのlsっぽいコマンド
[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)  
[Qiita記事](https://qiita.com/m_oba/items/828467a0f483e8dab845)  
## なにこれ？
**ls-Improved(lsi)** はディレクトリやファイルの一覧を表示する`ls`っぽいコマンドです。`ls`との違いはディレクトリの説明文も一緒に表示してくれることです。  
似たような名前のディレクトリをたくさん生成する場合などに、それらのディレクトリにそれぞれ特徴をメモしておくことで一覧性が良くなります。（機械学習の実験ディレクトリにパラメータをメモしたりして使っています。コマンドライン上で簡単に確認できるので便利です。）

### 使用例
下のような機械学習の実験ディレクトリがあったとします。  
しばしば、「このパラメータで実験したディレクトリはどれかなー。」とか「最良の結果を出した実験はどれだっけ」とかで迷うことがあります。  
そういうときは全ディレクトリをひとつひとつ確認したり、どこかのテキストファイルにすべての実験結果をメモしておいてそれを確認したりしますが、結構めんどくさいです。  

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/ls_using.png)

そういう背景があり**ls-Improved** を開発しました。このコマンドを使えば、下のようにどのディレクトリがどういったものか、ということが一目でわかります。

![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/lsi_using.png)


## Requirements
Python2.7とPython3.7で開発しました。おそらくPython2.7以上で動作します。それ以下は確認してないです。  
また、Pythonで動作するのでOSに依存しないはずです。

## Install
### pip install
```
# PyPI
pip install ls-Improved
```

### 手動でのインストール
pipする権限がない場合などは以下の手順でインストールしてください。  
1. githubのreleaseから最新のmanual install versionをダウンロードしてください。
`wget https://github.com/ShotaroKataoka/ls-Improved/archive/v0.3.0.beta0.manual.zip`
2. ダウンロードしたzipファイルを解凍してください。
3. 解凍したディレクトリの中にある`bin/`にパスを通します。
解凍した`bin/`を環境変数に追加するか、`bin/lsi`と`bin/mkdiri`をパスが通っている場所（`/usr/local/bin`など）に配置してください。

## 使い方
### 動作の仕組み
`lsi`は各ディレクトリにある`.description.lsi`というテキストファイルを読み取って、それをそのディレクトリの説明文として表示しています。
`.description.lsi`は単純なテキストファイルであり、`mkdiri`というコマンドによって生成されます。（他のプログラムやvimなどで生成しても問題ありません。）

### mkdiri
- `mkdiri DIRECTORY 'DESCRIPTION'` : DIRECTORYというディレクトリを作成し，'DESCRIPTION'という説明文を`.description.lsi`に作成  
- `mkdiri DIRECTORY` : ディレクトリを作成し，空の`.description.lsi`を作成  
- `mkdiri -a DIRECTORY DESCRIPTION` : 既存のディレクトリに説明文を上書き（`.description.lsi`が存在しない場合、新規作成する）  

**tips:**  
説明文には装飾をすることができます。  
- `\n` : コマンドライン上での生成時、改行を追加します。
- `;r;` `;g;` `;b;` `;w;` `;p;` : テキストの色を変更します。
- `;_;` : テキストに下線を追加します。
- `;e;` : 装飾を終了します。

**例**  

![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/mkdiri_decoration.png)

### lsi
- `lsi` : カレントディレクトリ内のファイルとディレクトリを表示  
- `lsi DIRECTORY` : パス内のファイルとディレクトリを表示  
- `lsi -a` : 隠れファイル・ディレクトリも表示  
- `lsi -F` : ファイルのみを表示
- `lsi -D` : ディレクトリのみを表示
- `lsi -s 'SEARCH_WORD'` : `SEARCH_WORD`でファイル名・説明文内を検索

## 関連
- [Emacs client](https://github.com/conao3/dired-lsi.el) by [conao3](https://github.com/conao3)
