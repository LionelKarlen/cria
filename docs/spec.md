# Cria
> Helpful additions to basic todo.txt

## Commands
### Format
Formats a file;
1. entries starting with "."
2. regular entries
3. completed entries
separated by an empty line between groups.

Usage:
`cria format <path>` -> in place format
`cria format --stdio <file>` -> format to standard output

#### Arguments
`--stdio`: used for helix and similar editors, that expect to take the file from standard input and output it to standard output.
