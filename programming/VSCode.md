- [General](#general)
  - [FiraCode](#firacode)
- [Extensions](#extensions)
  - [Github Markdown Preview](#github-markdown-preview)
  - [fzf fuzzy quick open](#fzf-fuzzy-quick-open)
  - [Auto Comment Blocks](#auto-comment-blocks)
  - [Back & Forth](#back--forth)
  - [Bookmarks](#bookmarks)
  - [Bracket Pair Colorizer 2](#bracket-pair-colorizer-2)
  - [Code Runner](#code-runner)
  - [Data Preview](#data-preview)
  - [Git Graph](#git-graph)
  - [Gremlins tracker](#gremlins-tracker)
  - [Indent rainbow](#indent-rainbow)
  - [Task Explorer](#task-explorer)
  - [nushell](#nushell)
- [List of recommended extensions](#list-of-recommended-extensions)

# General

## [FiraCode](https://github.com/tonsky/FiraCode)

Enable font ligatures

# Extensions

## [Github Markdown Preview](https://marketplace.visualstudio.com/items?itemName=bierner.github-markdown-preview)

Github like preview

## [fzf fuzzy quick open](https://marketplace.visualstudio.com/items?itemName=rlivings39.fzf-quick-open)

Environment settings (`.bashrc`) to:
 - use `fd`
 - find hidden files
 - colored output
```bash
export FZF_DEFAULT_COMMAND='fd --type file --hidden --follow --exclude .git --color=always'
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
export FZF_DEFAULT_OPTS="--ansi"
```

## [Auto Comment Blocks](https://marketplace.visualstudio.com/items?itemName=kevinkyang.auto-comment-blocks)

Simple extension for extended comments

## [Back & Forth](https://marketplace.visualstudio.com/items?itemName=nick-rudenko.back-n-forth)

Back and forth arrows to navigate the code

## [Bookmarks](https://marketplace.visualstudio.com/items?itemName=alefragnani.Bookmarks)

Code bookmarks

## [Bracket Pair Colorizer 2](https://marketplace.visualstudio.com/items?itemName=CoenraadS.bracket-pair-colorizer-2)

Colorize bracket pairs

## [Code Runner](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner)

Execute files and code snippets

## [Data Preview](https://marketplace.visualstudio.com/items?itemName=RandomFractalsInc.vscode-data-preview)

Preview structured data

## [Git Graph](https://marketplace.visualstudio.com/items?itemName=mhutchie.git-graph)

Graph view for repositories

## [Gremlins tracker](https://marketplace.visualstudio.com/items?itemName=nhoizey.gremlins)

Identify invisible or looking like legitimate characters

## [Indent rainbow](https://marketplace.visualstudio.com/items?itemName=oderwat.indent-rainbow)

Colorizes the indentation

## [Task Explorer](https://marketplace.visualstudio.com/items?itemName=spmeesseman.vscode-taskexplorer)

Sidebar for tasks

## [nushell](https://marketplace.visualstudio.com/items?itemName=TheNuProjectContributors.vscode-nushell-lang)

NuShell support

# List of recommended extensions

To install all extensions, save it to a file and execute this command:

```sh
cat vscode-extensions.list | xargs -L 1 code --install-extension
```

```
bierner.github-markdown-preview
rlivings39.fzf-quick-open
kevinkyang.auto-comment-blocks
nick-rudenko.back-n-forth
alefragnani.Bookmarks
CoenraadS.bracket-pair-colorizer-2
formulahendry.code-runner
RandomFractalsInc.vscode-data-preview
mhutchie.git-graph
nhoizey.gremlins
oderwat.indent-rainbow
spmeesseman.vscode-taskexplorer
TheNuProjectContributors.vscode-nushell-lang
```
