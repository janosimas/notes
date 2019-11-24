# Regex capturing tips

1. Notepad++ has a powerfull regex parser, better then VSCode.
2. VSCode and Notepad++ work in a different way for parenthesis in the substitution.

## Variable name

```regexp
([a-zA-Z]\w*)
```

## Indentation

```regexp
(^[^\S\r\n]*)
```

## Variable or raw string

```regexp
([a-zA-Z]\w*|"\w*")
```

## Anything except line break

```regexp
([^\n]*)
```
