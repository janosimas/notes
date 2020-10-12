# Table of contents
- [Table of contents](#table-of-contents)
- [Intro](#intro)
- [Tools](#tools)
- [Learning resources](#learning-resources)
- [Git](#git)
  - [Remove whitespace changes](#remove-whitespace-changes)

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
