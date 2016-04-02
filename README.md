# alltext

alltext is a command-line tool to debug strings. It is a friendlier version of `cat -ve`. Author uses it to learn how other command-line tools work: bash, vim, echo etc.

Like cat, alltext reads its input from stdin so it is to be used via pipes: like `tail -f file | alltext` or `echo -e string | alltext`.

cat:

```
$ printf "\x1b Hello \n" | cat -ve
^[ Hello $
$
```

alltext:

```
$ printf "\x1b Hello \n" | alltext
ESC Space H e l l o Space LF
$
```

### Installation

Currently alltext is available on Mac OS X but it should be easy to enable some other types of distribution supported by Rust/Cargo.

The one-file tar with alltext binary can be found on [Releases](https://github.com/stanislaw/alltext/releases) page.

### TODO

- Optional parameter to print string lengths (example: `A A A (3)`)

### Author

alltext is created by [Stanislav Pankevich](https://github.com/stanislaw).

### License

Released under MIT license, see `LICENSE` for more details.

