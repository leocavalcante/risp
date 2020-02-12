# risp

## Rand

Grabs random values from the given list.

```powershell
risp .\examples\input.csv rand 5 >> .\var\random_5.csv
```

## Pick

Picks a column from the given list

```powershell
risp .\examples\input.csv pick 3 >> .\var\emails.csv
```

---

```powershell
USAGE:
    risp.exe [OPTIONS] <input> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <delimiter>    File delimiter

ARGS:
    <input>    File path

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    pick    Pick a single column from the list
    rand    Gets random values from the list
```
