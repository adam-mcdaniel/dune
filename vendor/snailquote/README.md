## snailquote

[![Build Status](https://travis-ci.org/euank/snailquote.svg?branch=master)](https://travis-ci.org/euank/snailquote)

[![Docs](https://docs.rs/snailquote/badge.svg)](https://docs.rs/snailquote)

This library provides functions to escape and unescape strings.

It escapes them in a roughly 'sh' compatible way (e.g. double quotes supporting
backslash escapes, single quotes supporting no escapes).

In addition, it provides support for common c-like ascii escapes (like `\n` for
newline, `\v` for vertical tab, etc) and rust-string-like unicode (via
`\u{12ff}` style escapes).

More importantly, this library also provides the ability to un-escape a given
escaped text to recover the original string.

For more information on usage and what escape sequences will work, see the [docs](https://docs.rs/snailquote).

### Compatibility

snailquote intends to explicitly be compatible with the following use-cases:

1. Readable encoding of arbitrary strings for user's editing; re-parsing of
   said strings after being edited.
1. Reading and writing the values of [`os-release`](https://www.freedesktop.org/software/systemd/man/os-release.html) files.

Other files may have a similar shell-inspired format that snailescape works with, but it's up to you to verify it really is similar enough to be correct.

snailquote is inspired by, but not compatible with, the following:

1. gnulib [quotearg](https://www.gnu.org/software/gnulib/manual/html_node/Quoting.html), as used for 'ls' output -- snailquote handles unicode escapes differently.
2. ANSI-c string literal quoting -- snailquote handles unicode differently and supports shell-related escapes, like '$'.
3. sh string quoting -- snailquote handles unicode differently and is more lax about shell special characters.

### Why not use \<other library\>

Other libraries in rust I've found have one or more of the following problems:

1. The escaped text is not as easily human-editable
1. There is no way to un-escape text
1. NIH
