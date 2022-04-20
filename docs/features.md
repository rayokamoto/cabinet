# Features
This is an overview of the features that are currently available. 
#### *Disclaimer: Note that features may be added, revised, or removed at any time as this is in active development*

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
- `name`
- `size`

Other commands:
- `help`

Future commands:


### Path
You have two options when providing a path:
- Absolute path 
- Template path

When you run a command, you must provide a path/directory to sort. If no path option is provided, it defaults to using the absolute path. Otherwise, you can refer to the absolute path using the `-a` or `--absolute` options. Relative paths also work. E.g. you are in a directory with the folder `projects`, simply typing `projects` as your path will work.

Example
```
cab type /home/User/Downloads
```

If there are any spaces in the path to the folder, use quotation marks:
```
cab type "/home/User/Documents/Important Stuff" 
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

For help on a command, type `cab <command> --help`.

### date
```
Sort files by the date modified

Usage: cab date [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
    --before <date> Get files from before specified date. Date format is YYYY-MM-DD
    --after <date>  Get files from after specified date. Date format is YYYY-MM-DD
```
When sorting by date modified, you must provide either a before or after date, using the `--before` and `--after` options respectively.

#### Remarks
- The ability to use both these options to set a date range will be added in the future.

#### Examples
```
cab date /path/to/folder --after 2022-02-01
cab date /path/to/folder --before 2021-12-25
cab date -t downloads --after 2021-04-01
```

<br>

### name
```
Sort files by their name

Usage: cab name [<options>] <path>
    -p, --path          The path you are using is an absolute or relative path. This is the default option
    -t, --template      The path you are using is a predefined one. E.g. downloads for your downloads folder
    --includes <match>  File name includes...
    --excludes <match>  File name excludes...
```
You have two options: sort files that includes the given string OR sort files that DO NOT contain the given string. The matches are case-sensitive and quotation marks should be used if there are spaces in the string.

#### Remarks
- The ability to use both these options to set include and exclude patterns will be added in the future.

#### Examples
```
cab name /path/to/folder --includes Copy
cab name /path/to/folder --excludes important
cab name -t downloads --includes "hello world"
```

<br>

### size
```
Sort files by their size in KB (do not include 'KB' in the actual command)

Usage: cab size [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
    --lt <size>     Get files that are LESS THAN the specified size (in KB)
    --gt <size>     Get files that are GREATER THAN the specified size (in KB)
```
You must specify whether you want to sort files that are less than or greater than the file size you specified. Use `--lt` to sort by files less than the specified size and `--gt` for files greater than the specified size. Note that currently, only sizes in KB is supported. NOTE: do not include "KB" in the actual command!

#### Remarks
- The ability to use both the `--lt` and `--gt` options to set a size range will be added in the future.

#### Examples
```
cab size /path/to/folder --lt 1000
cab size /path/to/folder --gt 31
cab size -t downloads --gt 1729
```

<br>

### type
```
Sort files by file type

Usage: cab type [<options>] <path>
    -p, --path      The path you are using is an absolute or relative path. This is the default option
    -t, --template  The path you are using is a predefined one. E.g. downloads for your downloads folder
```
#### Remarks
- The option to specify only one file type may be added in the future.

#### Examples
```
cab type /path/to/folder
cab type -a /path/to/folder
cab type -t downloads
```
