# Table of contents
- [Table of contents](#table-of-contents)
- [Intro](#intro)
- [Tools](#tools)
- [Learning resources](#learning-resources)
- [Git](#git)
  - [Remove whitespace changes](#remove-whitespace-changes)
- [Decompose Hex flags](#decompose-hex-flags)

# Intro

Here are some resources that are not (programming-)language specific.

# Tools

- [Quicktype](https://quicktype.io/)
  - Generate language specific representation from a JSON example.

# Learning resources

This is not a list learning resources as they get outdated very fast, but here are some resources they will probably help and take some time to be outdated.

- [Exercism](https://exercism.io/)
  - Amazing resource for programming exercises.
    - Check answers
    - TDD model
    - Other solutions for comparisson (after you solve the exercise)

# Git

A few useful tips for using `git`.

## Remove whitespace changes

It's quite common for the `IDE` auto-format the code.

This command will commit only the **non-whitespace** changes.

```bash
git diff -U0 -w --no-color | git apply --cached --ignore-whitespace --unidiff-zero -
```

Reference: https://stackoverflow.com/a/45486981/1331436

# Decompose Hex flags

This is a python function to decompose a decimal number in a list of hex-flags:
```py
def hex_decompose(flag):
   b = bin(flag)[2:]
   for i in range(len(b)):
       if int(b[i]):
           x = len(b) - i - 1
           y = '1' + ('0' * x)
           print(hex(int(y,2)))
```

Reference: https://gist.github.com/earonesty/1023c9de69aa5ae02d83fe3baf4dd753
