# VSCode for C++ prototyping

- [Installation](#installation)
- [Requirements](#requirements)
- [Extensions](#extensions)
  * [C++](#c--)
  * [Debug](#debug)
  * [Project management](#project-management)
  * [Git](#git)
  * [Other](#other)
- [Prototyping](#prototyping)
  * [Building](#building)
    + [MSVC C++17](#msvc-c17)
    + [MSVC C++03](#msvc-c03)
    + [Clang C++17](#clang-c17)
    + [Clang C++03](#clang-c03)
  * [Executing and debug](#executing-and-debug)
- [Building projects](#building-projects)


# Installation

  You can install [**VSCode**](https://code.visualstudio.com/download) in 3 different ways:

  - User folder (recommended by **Microsoft**)
  - System wide
  - [Portable](https://code.visualstudio.com/docs/editor/portable).

  Each installation mode has its advantages. If you only have a specific need and uses **VSCode** for the same function, go with the user folder installation.

  The portable installation is useful to organize your extensions. It's useful if you work with more languages.

  Too many extensions can lag your **VSCode** and are hard to manage.

# Requirements

  The [MS C++](#c--) extension is a requirement for proper usage. You can replace some of its functionalities with other extensions but I recommend installing it.

# Extensions

## C++

### [C/C++ (Microsoft)](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)

  The **Microsoft** extension is the minimum one need to use C++.
  It works well but you'll need to set it up for anything more than a "folder project".

  You should create a configuration file with the `C\C++: Edit configurations` command. For more information about the configuration check the official [documentation](https://code.visualstudio.com/docs/cpp/config-msvc).

  In **Windows**, you may also need to configure the `CL` compiler path for different architectures.

### [Clangd (LLVM)](https://marketplace.visualstudio.com/items?itemName=llvm-vs-code-extensions.vscode-clangd)

  The official **LLVM** extension for **clangd** provides alternatives and extra tools for the Microsoft one.

  The extension can be set up by:
  1. `compile_commands.json`
      - More reliable
      - Adherent to the project configuration

<details>
<summary> Generating compile_commands </summary>

---

#### CMake

With *CMake*, enabling the flag `CMAKE_EXPORT_COMPILE_COMMANDS` will generate the `compile_commands.json`.

#### Visual Studio

*Visual Studio* is capable of generating the `compile_commands.json` file for internal usage. For that, you need at least `Visual Studio 2019 version 16.4`. You should enable `clang-tidy` in the [Code analysis](https://devblogs.microsoft.com/cppblog/code-analysis-with-clang-tidy-in-visual-studio/) section of your project and `Run Code Analysis` in your project or `Solution`.

You can just set the path to the file in your extensions or copy/link the file your project base folder.

If you have many projects in your solution, the script [merge_compile_commands.py](merge_compile_commands.py) can be used to merge them in a single file.

---

</details>

  2. manual configuration
      - Easy for the simple case
      - Complex with a lot of files for anything else

<details>
<summary> Clangd setup </summary>

---

  1. You need [Clang (9+)](http://releases.llvm.org/download.html) installed and available in your path.

  2. Add the settings  to disable the conflicts with MS extension:

 ```json
  "C_Cpp.autocomplete": "Disabled",
  "C_Cpp.formatting": "Disabled",
  "C_Cpp.errorSquiggles": "Disabled",
  "C_Cpp.intelliSenseEngine": "Disabled",
  "clangd.arguments": [
    "-clang-tidy-checks=-*,modernize-use-override",
    "--suggest-missing-includes",
    "--header-insertion=iwyu"
  ],
```

  3. [Optional] When using manual configuration, create a file with the compilation flags: [compile_flags.txt](compile_flags.txt)

---

</details>

### [Flylint](https://marketplace.visualstudio.com/items?itemName=jbenden.c-cpp-flylint)

  Flylint (C/C++ Advanced Lint) is an extension that integrates other tools in your `VSCode` to provide code analysis.

  Its integration with `clang` and `cppcheck` is set up minimum effort.

  One should disable the `clang` option when using another `clang-based` extension like `clangd`

<details>
<summary> Sample configuration </summary>

```json
"c-cpp-flylint.cppcheck.executable": "C:\\Program Files\\Cppcheck\\cppcheck.exe",
"c-cpp-flylint.flexelint.enable": false,
"c-cpp-flylint.clang.enable": true,
"c-cpp-flylint.clang.extraArgs": [
    "-Weverything",
    "-std=c++17",
    "-Wno-unused-macros",
    "-Wno-c++-compat"
],
"c-cpp-flylint.clang.includePaths": [
    "C:/Program Files (x86)/Microsoft Visual Studio/2019/Professional/VC/Tools/MSVC/14.22.27905/include",
],
```
</details>

### [Code Runner](https://marketplace.visualstudio.com/items?itemName=formulahendry.code-runner)

  Simple extension for building and executing a single file. Works with a lot of languages out of the box, with C++ you may need to tweak a little the command.
</details>

## Debug

### [C++ Set next statement](https://marketplace.visualstudio.com/items?itemName=ntoskrnl7.cxx-set-next-statement-extension)

  Debug super powers: Can jump the debugger to a different line.

## Project management

### [CMake](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cmake-tools)

  The **CMake** extension is very mature and full-featured. Automatic recognizes *CMakeList* files and configure the project.

### [Solution explorer](https://marketplace.visualstudio.com/items?itemName=fernandoescolar.vscode-solution-explorer)

  This unofficial extension has minimal features, you can open a project and navigate but not much more. One can create a file but filters do not work very well, it works better with folders. **No compilation.**

## Git

### [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)

  Git super powers. It has more features than most git GUI. In line blame, line history, search, you name it. More options than you can manage but don't worry, it works very well with the defaults.

### [Git Graph](https://marketplace.visualstudio.com/items?itemName=mhutchie.git-graph)

  The usual visualizer for the git branches. It's quite useful for an overview of the git branches or for searching.

## Other

### [Doxygen Documentation Generator](https://marketplace.visualstudio.com/items?itemName=cschlosser.doxdocgen)

  Auto generates skeleton documentation with arguments for functions and templates.

### [Auto Comment Blocks](https://marketplace.visualstudio.com/items?itemName=kevinkyang.auto-comment-blocks)

  Minor improvement for writing documentation blocks. It will auto-insert '*' when you break a line.

### [Better Readability](https://marketplace.visualstudio.com/items?itemName=pinage404.better-readability-extension-pack)

  Pack of extensions to improve readability. Will help bracket matching, colorize the indentation and mark text gremlins for example.

# Prototyping

- [Building](#building)
  * [MSVC C++17](#msvc-c17)
  * [MSVC C++03](#msvc-c03)
  * [Clang C++17](#clang-c17)
  * [Clang C++03](#clang-c03)
- [Executing and debug](#executing-and-debug)

## Building
  These tasks allow you to build the current file with the desired compiler.

  To use these tasks you need to alter the `tasks.json`. You can create a fresh one with the `Configure task` command in `VSCode`. Add the tasks you need to your `tasks.json` in the `"tasks": [...]` entry.

  Another option is to copy the sample file with all the tasks: [tasks.json](tasks.json)

  The binaries will be created in the `build/` folder.

  The `create-build-folder` task is required by the other tasks and is called automatically:
```json
{
    "label": "create-build-folder",
    "type": "process",
    "command": "cmd",
    "args": [
        "/C mkdir build\\ | echo "
    ],
    "presentation": {
        "reveal":"never",
        "echo": false
    }
}
```
Keybinding to build the current file: `ctrl+shift+b`

### MSVC C++17
```json
{
    "label": "MSVC C++17",
    "type": "shell",
    "command": "cl.exe",
    "args": [
        "/EHsc",
        "/FC",
        "/Od",
        "/permissive-",
        "/std:c++17",
        "/W4",
        "/Z7",
        "/Fdbuild/${fileBasenameNoExtension}.pdb",
        "/Febuild/${fileBasenameNoExtension}.exe",
        "/Fobuild/${fileBasenameNoExtension}.obj",
        "${file}"
    ],
    "group": "build",
    "presentation": {
        "reveal":"always"
    },
    "problemMatcher": "$msCompile",
    "dependsOrder": "sequence",
    "dependsOn":["create-build-folder"]
}
```

### MSVC C++03
```json
{
    "label": "MSVC C++03",
    "type": "shell",
    "command": "cl.exe",
    "args": [
        "/EHsc",
        "/FC",
        "/Od",
        "/permissive-",
        "/W4",
        "/Z7",
        "/Fdbuild/${fileBasenameNoExtension}.pdb",
        "/Febuild/${fileBasenameNoExtension}.exe",
        "/Fobuild/${fileBasenameNoExtension}.obj",
        "${file}"
    ],
    "group": "build",
    "presentation": {
        "reveal":"always"
    },
    "problemMatcher": "$msCompile",
    "dependsOrder": "sequence",
    "dependsOn":["create-build-folder"]
}
```

### Clang C++17
```json
{
    "label": "Clang C++17",
    "type": "shell",
    "command": "clang-cl.exe",
    "args": [
        "-m32",
        "/EHsc",
        "/FC",
        "/Od",
        "/permissive-",
        "/std:c++17",
        "/W4",
        "/Z7",
        "/Fdbuild/${fileBasenameNoExtension}.pdb",
        "/Fobuild/",
        "/Febuild/${fileBasenameNoExtension}.exe",
        "${file}"
    ],
    "group": "build",
    "presentation": {
        "reveal":"always"
    },
    "problemMatcher": "$msCompile",
    "dependsOrder": "sequence",
    "dependsOn":["create-build-folder"]
}
```

### Clang C++03
```json
{
    "label": "Clang C++03",
    "type": "shell",
    "command": "clang-cl.exe",
    "args": [
        "-m32",
        "/EHsc",
        "/FC",
        "/Od",
        "/permissive-",
        "/W4",
        "/Z7",
        "/Fdbuild/${fileBasenameNoExtension}.pdb",
        "/Fobuild/",
        "/Febuild/${fileBasenameNoExtension}.exe",
        "${file}"
    ],
    "group": "build",
    "presentation": {
        "reveal":"always"
    },
    "problemMatcher": "$msCompile",
    "dependsOrder": "sequence",
    "dependsOn":["create-build-folder"]
}
```

## Executing and debug

This will allow you to execute and debug a file build with any of the tasks in the last item.

As with the tasks, you'll need a `launch.json`, you can generate a default one with the command: `Debug: open launch.json`.

Add the configuration to the `"configurations": [...]` entry or use the [launch.json](launch.json) sample configuration.
```json
{
    "name": "Debug current file",
    "type": "cppvsdbg",
    "request": "launch",
    "program": "${workspaceFolder}/build/${fileBasenameNoExtension}.exe",
    "args": [],
    "stopAtEntry": true,
    "cwd": "${workspaceFolder}/build",
    "environment": [],
    "externalConsole": true
}
```

# Building projects

## CMake
  Using the [CMake](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cmake-tools) extension it will setup the building environment for you.

## MSBuild
  MSBuild is a different thing, you may have to do some tweaking when building bigger projects but here are the basics:

```json
{
    "label": "MS: Build",
    "command": "msbuild",
    "type": "process",
    "args": [
        "/m",
        "/property:GenerateFullPaths=true",
        "/p:Configuration=Debug;Platform=Win32"
    ],
    "group": "build",
    "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": true,
        "panel": "shared",
        "showReuseMessage": true,
        "clear": true
    },
    "problemMatcher": [
        "$msCompile"
    ]
},
{
    "label": "MS: Clean",
    "command": "msbuild",
    "type": "process",
    "args": [
        "-t:Clean",
        "/p:Configuration=Debug;Platform=Win32",
        "/property:GenerateFullPaths=true"
    ],
    "group": "build",
    "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": true,
        "panel": "shared",
        "showReuseMessage": true,
        "clear": true
    },
    "problemMatcher": [
        "$msCompile"
    ]
}
```
