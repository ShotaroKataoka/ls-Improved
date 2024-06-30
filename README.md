# ls-Improved: Descriptive ls-like Command in Rust

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/lsi.png)

[![Downloads](https://pepy.tech/badge/ls-improved)](https://pepy.tech/project/ls-improved)

[日本語 README](https://github.com/ShotaroKataoka/ls-Improved/blob/master/README.ja.md)

## What is it?

**ls-Improved (lsi)** is a command-line tool for listing directory structures along with their descriptions. It is particularly useful for managing and accessing experimental results, which can often be buried deep within multiple directories.

### Example

Consider a directory structure filled with experimental results. Finding the best result can be cumbersome, often requiring you to open multiple directories or maintain a separate summary file.

![ls](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/ls_using.png)

With **ls-Improved (lsi)**, you can view the list of directories along with their descriptions in a single command:

![lsi](https://github.com/ShotaroKataoka/ls-Improved/blob/master/doc/images/lsi_using.png)

## Install

`Follow these steps to download the latest version of the binary file and set up the PATH:`

1. Visit the GitHub releases page.
2. Find the latest version release and download the corresponding binary file.
3. Place the downloaded binary file in an appropriate directory (e.g., `/usr/local/bin`).
4. Add the directory to your environment variables. Typically, you can do this by adding the following line to your `~/.bashrc` or `~/.zshrc` file:
   ```sh
   export PATH=$PATH:/usr/local/bin
   ```
5. To apply the changes, restart your terminal or run the following command:
   ```sh
   source ~/.bashrc  # or source ~/.zshrc
   ```

With these steps, the installation will be complete, and you will be able to use the binary file from the command line.

## Usage

Below are the usage details:

### Basic Commands

- `lsi [PATH]`: Show the directory structure and descriptions of the specified `PATH` (default is `"./"`).
- `lsi -a, --all`: Include entries that start with `.` (hidden files and directories).
- `lsi -f, --only-files`: Show only files, not directories.
- `lsi -d, --only-dirs`: Show only directories, not files.
- `lsi -c, --config-path <ConfigPath>`: Load configuration from the specified `ConfigPath`.
- `lsi -n, --line-num <Number>`: Limit the description lines to the specified `Number`.
- `lsi -S, --sort-mode <Mode>`: Sort by path (`p`) or description (`d`).

### Managing Descriptions

- `lsi -s, --set-description <Description> [PATH]`: Write the specified `Description` to the `.description.lsi` file in the specified `PATH`.
- `lsi -e, --edit-description <Editor> [PATH]`: Open the description `.description.lsi` file in the specified `Editor` (default is `vim`).

Example command to set a description:

```sh
lsi -s "This is a brief description" ./experiments/run1
```

Example command to edit a description using nano:

```sh
lsi -e nano ./experiments/run1
```

## Configuration Tips

It is beneficial to add the following aliases to your `.bashrc` or `.zshrc` for quicker navigation and usage:

```sh
alias clear='clear && lsi ./'
function cdlsi () {
    cd "$@" && lsi ./
}
alias cd='cdlsi'
```

## Related Projects

- [Emacs client](https://github.com/conao3/dired-lsi.el) by [conao3](https://github.com/conao3)

Feel free to explore, contribute, and open issues if you encounter any. Happy organizing!
