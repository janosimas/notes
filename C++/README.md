# C++

## General

1. [Regex](regex.md) tips for refactoring code

2. [VSCode](vscode/vscode.md) setup for C++

3. [Clang tidy diff](clang-tidy-diff.py)

  This script allow `clang-tidy` to be applied only to modified files and lines. This allows a team to use `clang-tidy` in large codebases and still only get meaningfull errors for the newly modified code.

   Sample command:
```bash
git diff -U0 HEAD^ | clang-tidy-diff.py -strip 1 -- -checks=-*,modernize-use-override
```

  The `clang-tidy-diff.py` is a modified version of the official script that fix a few issues. This version allows one to pass arguments directly to `clang-tidy` using the '`--`' flag.

## Ubuntu
  1. To use newer versions of `clang` you have to update your environment with all the tools and binary names, the script: [clang_update_alternativs.sh](clang_update_alternativs.sh) updates the system to the desired version. **Update the script with the desired version**.
