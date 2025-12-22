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

### Archive
Archive all completed tasks from a file.

Usage:
`cria archive <path> --to <path>` -> write all completed tasks from the first file to the second.

#### Arguments
`--to <path>`: the path of the file to which the completed tasks should be prepended
`--no-strip`: does not remove the `x` literal denoting a completed task
`--date`: the date literal to be prepended to the task. Defaults to YYYY-MM-DD
