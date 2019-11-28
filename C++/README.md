# C++
  * [Refactoring](#refactoring)
  * [Text editor (VSCode)](#text-editor-vscode)
  * [Clang tidy](#clang-tidy)
    + [Clang-tidy-diff](#clang-tidy-diff)
  * [Clang format](#clang-format)
    + [Clang-format-diff](#clang-format-diff)
  * [Ubuntu](#ubuntu)

## Refactoring
  Some tips for using regular expressions for refactoring can be found in [regex.md](regex.md).

  Also, take a look in the usage of [Clang tidy](#clang-tidy) and [VSCode](#text-editor) with the `clangd` extension.

## Text editor (VSCode)
  [VSCode](vscode/vscode.md) setup for C++

## Clang tidy

  `Clang-tidy` is tool in the `clang` suite for checking the code. It checks static issues, bug-prone styles and patterns. There are thousands of possible check, start with this sugested list of checks and modify as needed.
  ```
  -checks=-*,bugprone-*,cert-*, clang-*,cppcoreguidelines-*,llvm-*,misc-*,modernize-*,performance-*,readability-*,-modernize-use-trailing-return-type
  ```

### Clang tidy diff
  The [clang-tidy-diff.py](clang-tidy-diff.py) allow `clang-tidy` to be applied only to modified files and lines. This allows a team to use `clang-tidy` in large codebases and still only get meaningfull errors for the newly modified code.

   Sample command:
```bash
git diff -U0 HEAD^ | clang-tidy-diff.py -strip 1 -- -checks=-*,modernize-use-override
```

  The `clang-tidy-diff.py` is a modified version of the official script that fix a few issues. This version allows one to pass arguments directly to `clang-tidy` using the '`--`' flag.

## Clang format

  The `clang-format` tool is a higly configurable tool for formatting `C++` code. It's very stable and will (probably) not break your code if it was not already broken.

  A configuration file can be generated with a preview in [clang-format-configurator](https://zed0.co.uk/clang-format-configurator/).

  `Visual Studio` suports `clang-format` by default.

### Clang format diff

  This script allow `clang-format` to be applied only in the modified area of the code. This allows a team to use `clang-format` in large codebases and still only commit formatted code.

```bash
git diff -U0 --no-color HEAD^ | clang-format-diff.py -p1 -i
```

## Ubuntu
  1. To use newer versions of `clang` you have to update your environment with all the tools and binary names, the script: [clang_update_alternativs.sh](clang_update_alternativs.sh) updates the system to the desired version. **Update the script with the desired version**.
