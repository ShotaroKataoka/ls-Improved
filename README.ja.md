# ls-Improved : 説明文つきの ls っぽいコマンド

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/lsi.png)

[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)  
[Qiita 記事](https://qiita.com/m_oba/items/828467a0f483e8dab845)

## なにこれ？

**ls-Improved(lsi)** はディレクトリやファイルの一覧を表示する`ls`っぽいコマンドです。`ls`との違いはディレクトリの説明文も一緒に表示することです。  
似たような名前のディレクトリをたくさん生成する場合などに、それぞれのディレクトリに特徴をメモしておくことで一覧性が向上します。（機械学習の実験ディレクトリにパラメータをメモする際などに便利です。）

### 使用例

下のような機械学習の実験ディレクトリがあったとします。  
しばしば、「このパラメータで実験したディレクトリはどれかな？」とか「最良の結果を出した実験はどれだっけ？」などとなることがあります。  
全ディレクトリをひとつひとつ確認したり、実験結果をどこかのテキストファイルにメモしておくのは手間がかかることがあります。

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/ls_using.png)

この背景から**ls-Improved** を開発しました。このコマンドを使うと、以下のように各ディレクトリの内容が一目でわかります。

![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/lsi_using.png)

## インストール

`最新バージョンのバイナリファイルをダウンロードして、パスを設定する手順は以下の通りです。`

1. GitHub のリリースページにアクセスします。
2. 最新バージョンのリリースを見つけて、自分の OS と対応するバイナリの zip ファイルをダウンロードします。
3. ダウンロードした zip ファイルを解凍します。
   ```sh
   unzip <file>.zip
   ```
4. 解凍してできたバイナリファイルを適切なディレクトリに配置します（例: `/usr/local/bin`）。
5. 環境変数にパスを追加します。一般的な方法として、以下の内容を`~/.bashrc`または`~/.zshrc`に追加します。
   ```sh
   export PATH=$PATH:/usr/local/bin
   ```
6. 変更を反映させるためにターミナルを再起動するか、以下のコマンドを実行します。
   ```sh
   source ~/.bashrc  # または source ~/.zshrc
   ```

これでインストールが完了し、コマンドラインからバイナリファイルを使用できるようになります。

## 使い方

以下に各コマンドの使用例を示します。

### 基本コマンド

- `lsi [PATH]`: 指定された`PATH`（デフォルトは`"./"`）のディレクトリ構造と説明文を表示します。
- `lsi -a, --all`: `.`で始まるエントリ（隠しファイル・ディレクトリ）を含めて表示します。
- `lsi -f, --only-files`: ファイルのみを表示します（ディレクトリは表示しません）。
- `lsi -d, --only-dirs`: ディレクトリのみを表示します（ファイルは表示しません）。
- `lsi -c, --config-path <ConfigPath>`: 指定した`ConfigPath`から設定を読み込みます。
- `lsi -n, --line-num <Number>`: 説明文の行数を指定した`Number`に制限します。
- `lsi -S, --sort-mode <Mode>`: パス (`p`) または 説明 (`d`) でソートします。

### 説明文の管理

- `lsi -s, --set-description <Description> [PATH]`: 指定された`PATH`の `.description.lsi` ファイルに`Description`を書き込みます。
- `lsi -e, --edit-description <Editor> [PATH]`: 指定された`PATH`の `.description.lsi` ファイルを`Editor`（デフォルトは`vim`）で開きます。

説明文を設定する例:

```sh
lsi -s "これは簡単な説明です" ./experiments/run1
```

nano で説明文を編集する例:

```sh
lsi -e nano ./experiments/run1
```

## 設定のヒント

以下のエイリアスを`.bashrc`や`.zshrc`に追加すると、より便利に使えます：

```sh
alias clear='clear && lsi ./'
function cdlsi () {
    cd "$@" && lsi ./
}
alias cd='cdlsi'
```

## 関連プロジェクト

- [Emacs クライアント](https://github.com/conao3/dired-lsi.el) by [conao3](https://github.com/conao3)

試してみてください。バグ報告やフィードバックもお待ちしています。
