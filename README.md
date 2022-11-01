# RustNote
RustNote is a simple command line application that helps you take and display notes.

## How does it work?
RustNote takes at most one argument. Running RustNote with no arguments will display today's notes.
Running RustNote with an argument will add a note to today's note file.

Example usage:

```shell
$> rnote
2022-10-01 08:00:00 - Make coffee
2022-10-01 08:15:08 - Grab another coffee
2022-10-01 08:19:10 - Write code
```

Running RustNote with an argument adds it to your list of notes.

```shell
$> rnote "Add note"
$> rnote
2022-10-01 08:00:00 - Make coffee
2022-10-01 08:15:08 - Grab another coffee
2022-10-01 08:19:10 - Write code
2022-10-01 08:20:42 - Add note
```

## Where does RustNote store my note files?

Note files are stored in the user's home directory in a `.notes` subdirectory.
Notes file names are formatted as such `notes-<date>.txt`.

You can pass the argument `--path` in order to determine where RustNote has chosen to store these files.

```shell
$> rnote --path
/home/my_user/.notes
```