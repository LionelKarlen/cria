# Cria

# About
<!-- Paragraph of what it does. What are the major features? What does it solve? -->
Cria is a simple, editor-agnostic formatting tool for todotxt. Cria distinguishes between 3 types of tasks:
- Marked tasks (task starts with `. `)
- Regular tasks
- Completed tasks (task starts with `x `)

Marked tasks imply importance or priority, and serve as a reminder for what was being worked on. They are therefore moved to the top by Cria. Regular tasks serve simply as a list of tasks to be completed. They are left as-is by Cria. Completed tasks are (usually) temporary markers of tasks that have been completed and can be archived or deleted. Cria moves them to the bottom of your file, with a newline separating them from the active of the tasks.

All further language features are ignored by Cria. A full specification of the language is available at [todotxt.org](https://github.com/todotxt/todo.txt). 

# Getting started
<!-- How do I install? Keep this as short as possible. -->
Cria can simply be `cargo install`-ed from the repo.

<!-- How do I use it at a glance? What is the minimum I need to know to get it running? -->
The help dialogue for the format commands reads as follows:

```
Format the given file with `marked` entries first, regular entries second, completed entries last

Usage: cria format <FILE|--stdio>

Arguments:
  [FILE]  The file to format

Options:
  -s, --stdio  Specify that text input comes from stdin, and the sorted content is given in stdout
  -h, --help   Print help

```

All that is left is to enable the formatter in your editor, and point it to the Cria command.

# Contributions guide
If you are interested in contributing, please fork the repo and open a pull request with your feature or fix.

# Acknowledgements
- todotext.org
