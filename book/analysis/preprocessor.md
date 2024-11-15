# Preprocessor

HEMTT will provide warnings for common issues in your config, in both the preprocessing and rapifying stages.

## Warning Suppression

Currently, HEMTT only allows the suppression of certain preprocessor warnings. To suppress a warning, use the following structure:

```cpp
#pragma hemtt suppress { warning code } { scope = line }
```

The warning code can be one of the following:

| Code | Description |
| ---- | ----------- |
| pw3_padded_arg | Padded argument in a macro call |

The scope can be one of the following, if not specified, the scope will be `line`.

| Scope | Description |
| ----- | ----------- |
| line | Suppresses the warning for the next line |
| file | Suppresses the warning for the remainder of the current file, not including includes |
| config | Suppresses the warning for the remainder of the current config, including includes |

## Preprocessor Flags

HEMTT provides a few preprocessor flags to control the behavior of the preprocessor.

| Flag | Description |
| ---- | ----------- |
| pw3_ignore_format | Ignores padded arguments in `ARR_N`, `WARNING_N`, `TRACE_N`, `FORMAT_N`, etc. macros |
| pe23_ignore_has_include| Assume any `#if __has_include` is false |

The scope of these flags is the same as the warning suppression scope.

## Preprocessor Warnings

### [PW1] Redefine Macro

This warning is emitted when a macro is defined more than once.

```cpp
#define FOO 1
#define FOO 2
```

It may also appear when a macro is defined in a file that is included more than once.

```cpp
// foo.hpp
#define FOO 1

// bar.hpp
#include "foo.hpp"
#include "foo.hpp"
```

### [PW2] Invalid Config Case

This warning is emitted when `config.cpp` is not all lowercase, e.g. `Config.cpp`.

### [PW3] Padded Argument

This warning is emitted when an argument to a macro is padded with spaces.

```cpp
#define Introduction(var1, var2) var1, meet var2
HELLO(Jim, Bob)
```

This would produce `Jim, meet  Bob` instead of `Jim, meet Bob`. (Note the extra space before `Bob`).

By default, all macros are checked, but a flag can be set to ignore `ARR_N`, `WARNING_N`, `TRACE_N`, `FORMAT_N`, etc. macros.

```cpp
#pragma hemtt flag pw3_ignore_format { scope = line }
```
