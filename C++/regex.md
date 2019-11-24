# Regex capturing tips

1. Notepad++ has a powerfull regex parser, better then VSCode.
2. VSCode and Notepad++ work in a different way for parenthesis in the substitution.

## Variable name

```
([a-zA-Z]\w*)
```

## Indentation

```
(^[^\S\r\n]*)
```

## Variable or raw string

```
([a-zA-Z]\w*|"\w*")
```

## Anything except line break

```
([^\n]*)
```
