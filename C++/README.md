# C++
  * [Usefull tricks](#usefull-tricks)
  * [Refactoring](#refactoring)
  * [Text editor (VSCode)](#text-editor-vscode)
  * [CppCheck](#cppcheck)
  * [Clang tidy](#clang-tidy)
    + [Clang-tidy-diff](#clang-tidy-diff)
  * [Clang format](#clang-format)
    + [Clang-format-diff](#clang-format-diff)
  * [Ubuntu](#ubuntu)

## Usefull tricks

### How to find the size of a structure during programming in visual studio


If you hover over foo, it will show `4Ui64` - the size of x is 4. The Ui64 suffix is because size_t is Unsigned, Integral and 64 bits. Since it uses Intellisense, you don't need to compile the code. You can put `Sizer` in your precompiled header.

```cpp
template <size_t S> class Sizer{};
int x;
Sizer<sizeof(x)> foo;
```

Reference: https://stackoverflow.com/a/24776469

### Optional back inserter

The interator helps address the composability issues of the standart library. In a case where you want to apply a filter and transform in a container, this would help not just the composability but also usage in non copiable values.

```cpp
template<class Container>
struct optional_back_insert_iterator
: public std::iterator< std::output_iterator_tag,
void, void, void, void >
{
    explicit optional_back_insert_iterator( Container& c )
    : container(std::addressof(c))
    {}

    using value_type = typename Container::value_type;

    optional_back_insert_iterator<Container>&
    operator=( const boost::optional<value_type> opt )
    {
        if (opt) {
            container->push_back(std::move(opt.value()));
        }
        return *this;
    }

    optional_back_insert_iterator<Container>&
    operator*() {
        return *this;
    }

    optional_back_insert_iterator<Container>&
    operator++() {
        return *this;
    }

    optional_back_insert_iterator<Container>&
    operator++(int) {
        return *this;
    }

protected:
    Container* container;
};

template<class Container>
optional_back_insert_iterator<Container> optional_back_inserter(Container& container)
{
    return optional_back_insert_iterator<Container>(container);
}
```

Reference: https://stackoverflow.com/a/34132758

### value_type of a c-array

When working with templated containers, sometimes we need the type of the element of the container. The usual way for standart containers is to use `Container::value_type`. The problem is that `C-array` does not have a `::value_type`.

Some improvements can be found on the link.

```cpp
template<typename T>
using element_type_t = std::remove_reference_t<decltype(*std::begin(std::declval<T&>()))>;
```

Reference: https://stackoverflow.com/a/44522730

## Refactoring
  Some tips for using regular expressions for refactoring can be found in [regex.md](regex.md).

  Also, take a look in the usage of [CppCheck](#cppcheck), [Clang tidy](#clang-tidy),  [Clang format](#clang-format) and [VSCode](#text-editor) with the `clangd` extension.

## Text editor (VSCode)
  [VSCode](vscode/vscode.md) setup for C++

## CppCheck

  The [Cppcheck](http://cppcheck.sourceforge.net/) is a static analysis tool that identifies many of the most common bugs like usage of an uninitialized variable. It has a `GUI` and can be integrated with most editors and `IDE` with extensions.

  It does not depend on your build system.

## Clang-tidy

  `Clang-tidy` is tool in the `clang` suite for checking the code. It checks static issues, bug-prone styles and patterns. There are thousands of possible check, start with this suggested list of checks and modify as needed.
  ```
  -checks=-*,bugprone-*,cert-*, clang-*,cppcoreguidelines-*,llvm-*,misc-*,modernize-*,performance-*,readability-*,-modernize-use-trailing-return-type
  ```

### Clang-tidy-diff
  The [clang-tidy-diff.py](clang-tidy-diff.py) allows `clang-tidy` to be applied only to modified files and lines. This allows a team to use `clang-tidy` in large codebases and still only get meaningful errors for the newly modified code.

   Sample command:
```bash
git diff -U0 HEAD^ | clang-tidy-diff.py -strip 1 -- -checks=-*,modernize-use-override
```

  The `clang-tidy-diff.py` is a modified version of the official script that fixes a few issues. This version allows one to pass arguments directly to `clang-tidy` using the '`--`' flag.

## Clang format

  The `clang-format` tool is a highly configurable tool for formatting `C++` code. It's very stable and will (probably) not break your code if it was not already broken.

  A configuration file can be generated with a preview in [clang-format-configurator](https://zed0.co.uk/clang-format-configurator/).

  `Visual Studio` supports `clang-format` by default.

### Clang format diff

  This script allows `clang-format` to be applied only in the modified area of the code. This allows a team to use `clang-format` in large codebases and still only commit formatted code.

```bash
git diff -U0 --no-color HEAD^ | clang-format-diff.py -p1 -i
```

## Ubuntu
  1. To use newer versions of `clang` you have to update your environment with all the tools and binary names, the script: [clang_update_alternatives.sh](clang_update_alternatives.sh) updates the system to the desired version. **Update the script with the desired version**.
