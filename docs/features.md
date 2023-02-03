# Features
This is an overview of the features that are currently available.

***Disclaimer: Note that features may be added, revised, or removed at any time as this is in active development***

---

## Overview

In general, the way commands work is as follows:
- Type the command (method you want to sort by)
- Provide a path to sort (using normal or template)
- Add any other required/optional arguments

### Usage
```
cab <command> [options] [<path>]
```

### List of Commands
Currently implemented commands:
- `date`
- `multisort`
- `name`
- `size`
- `type`

Other commands:
- `help`

Future commands:

### Path
You have two options when providing a path:
- Normal path 
- Template path

When you run a command, you must provide a path/directory to sort. Relative paths also work - e.g. if you are in a directory with the folder `projects`, simply typing `projects` as your path will work.

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
* `documents`
* `downloads`
* `desktop`
* `home` (e.g. `C:\Users\User\` or `/home/User/`)
* `music` (or `audio`)
* `pictures`
* `videos` (or `movies`)

Example
```
cab type -t downloads
```
This will sort the user's downloads folder

### Other arguments
These are command-specific and are documented under the [commands](#commands) section.

---

## Commands
A general overview of the commands available.

For help on a command, type `cab help <command>`.

### date
```
Sort files by their date of modification

Usage: cab date [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -B, --before <date>    Get files from before the specified date. Date format is YYYY-MM-DD
  -A, --after <date>     Get files from after the specified date. Date format is YYYY-MM-DD
  -t, --template         The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)
  -o, --output <output>  Specify the name of the output folder
  -h, --help             Print help
```
When sorting by date modified, you must provide either a before or after date, or both, using the `--before` and `--after` options respectively.

#### Examples
```
cab date /path/to/folder --after 2022-02-01
cab date /path/to/folder --before 2021-12-25
cab date -t downloads --after 2021-04-01 --before 2022-02-01
```

<br>

### multisort
```
Sort files using multiple file attributes

Usage: cab multisort [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -B, --before <date>     Get files from before the specified date. Date format is YYYY-MM-DD
  -A, --after <date>      Get files from after the specified date. Date format is YYYY-MM-DD
  -I, --includes <match>  File name includes...
  -E, --excludes <match>  File name excludes...
  -m, --min <size>        Get files that are GREATER THAN the specified size (in KB)
  -M, --max <size>        Get files that are LESS THAN the specified size (in KB)
  -o, --output <output>   Specify the name of the output folder
  -T, --type <file-type>  Sort files according to the specific file type
  -t, --template          The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)
  -h, --help              Print help
```

#### Examples
```
cab multisort --before 2023-01-01 --type "txt" --template documents
cab multisort -m 10 -M 1000 -E "Copy of" /path/to/folder
```

<br>

### name
```
Sort files by file name

Usage: cab name [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -I, --includes <match>  File name includes...
  -E, --excludes <match>  File name excludes...
  -t, --template          The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)
  -o, --output <output>   Specify the name of the output folder
  -h, --help              Print help
```
Sort files that includes the given string OR sort files that DO NOT contain the given string. Both options can be provided at once, although at least one must be provided. The matches are case-sensitive and quotation marks should be used if there are spaces in the string.

#### Examples
```
cab name /path/to/folder --includes Copy
cab name /path/to/folder --excludes important
cab name -t downloads --includes "hello world" --excludes "earth"
```

<br>

### size
```
Sort files by their size in KB (do not include 'KB' in the actual command)

Usage: cab size [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -m, --min <size>       Get files that are GREATER THAN the specified size (in KB)
  -M, --max <size>       Get files that are LESS THAN the specified size (in KB)
  -t, --template         The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)
  -o, --output <output>  Specify the name of the output folder
  -h, --help             Print help
```
Specify whether you want to sort files that are less than or greater than the file size you specified. Use `--max` to sort by files less than the specified size and `--min` for files greater than the specified size. Currently, only sizes in KB is supported. NOTE: do not include "KB" in the actual command!

#### Remarks
- The ability to use a size other than KB (e.g. MB, GB) will be added in the future.

#### Examples
```
cab size /path/to/folder --max 1000
cab size /path/to/folder --min 31
cab size -t downloads --min 10 --max 10000
```

<br>

### type
```
Sort all or select files by file type

Usage: cab type [OPTIONS] <PATH>

Arguments:
  <PATH>

Options:
  -T, --type <file-type>  Sort files according to the specific file type
  -t, --template          The path you are using is a predefined one (e.g. 'downloads' for your downloads folder)
  -o, --output <output>   Specify the name of the output folder
  -h, --help              Print help
```
#### Remarks
- The option to specify only one file type will be added in the future.

#### Examples
```
cab type /path/to/folder
cab type -t downloads
```
