# Features
This is an overview of the features that are currently available. 
#### *Disclaimer: Note that features can be added, revised, or removed at any time as this is in active development*

<br>

---

## Overview

In general, the way commands work is as follows:
- Type the command (method you want to sort by)
- Provide a path to sort (use absolute or template)
- Add any other required/optional arguments

### List of Commands
Currently implemented commands:
- `type`
- `date`

Other commands:
- `help`

Future commands:
- `name`
- `size`


### Path
You have two options when providing a path:
- Absolute path 
- Template path

When you run a command, you must provide a path/directory to sort. If no path option is provided, it defaults to using the absolute path. Otherwise, you can refer to the absolute path using the `-a` or `--absolute` options. Currently, the option to use relative paths is not supported. E.g. you are in a directory with the folder `projects`, simply typing `projects` as your path will not work. 

Example
```
$ cab type /home/User/Downloads
```

<br>

You can also use predefined templates to save you time from typing out the whole directory. Invoke using the `-t` or `--template` option.

Currently, the following templates are available:
* Documents
* Downloads
* Desktop
* Home directory (e.g. `C:\Users\User\` or `/home/User/`)
* Music (or Audio)
* Pictures
* Videos (or Movies)

Example
```
cab type -t downloads
```
This will sort the user's downloads folder

### Other arguments
These are command-specific and are documented under the [commands](#commands) section.


<br>

---

## Commands
A general overview of the commands available.

### type
Usage
```
cab type [<options>] <path>
```


### date
Usage
```
cab date [<options>] <path>
```
