# Cabinet
A convenient, cross-platform file sorting system that sorts files based on their attributes, such as file type, file name, and date modified.

#### *Disclaimer: Note that features may be added, revised, or removed at any time as this is in active development*

---

## Installation
Clone the repository 
```
git clone https://github.com/rayokamoto/cabinet
cd cabinet
```
Build with `cargo`
```
cargo build --release
```

## Usage
```
cab <command> [options] [<path>]
```

<h3 align="center">
    More in-depth information is available in the <a href="/docs/features.md">features documentation</a>.
</h3>

#### Commands
* `type` - Sort by file type (e.g. `.jpg`, `.png`, `.pdf`, etc.)
* `date` - Sort by date modified (before or after given date in `YYYY-MM-DD` format)
* `name` - Sort by name of file (by whether given pattern is in the name or not)
* `size` - Sort by size of file in KB (by whether file is smaller or larger than given size)

There are two options for sorting files: normal paths and path templates:
#### Using normal paths
When you run a command, you must provide a path/directory to sort. Relative paths also work - e.g. if you are in a directory with the folder `projects`, simply typing `projects` as your path will work.

No flags provided will default to using normal paths:
```
cab type "C:\Users\User\Downloads"
cab type "/home/User/Downloads"
```
#### Using templates
With templates, you can sort directories quickly. Invoke using the `-t` or `--template` option:
```
cab type -t downloads
```
Currently, the following templates are available:
* `documents`
* `downloads`
* `desktop`
* `home` (e.g. `C:\Users\User\` or `/home/User/`)
* `music` (or `audio`)
* `pictures`
* `videos` (or `movies`)

## Testing
For testing, such as generating test files to sort, Python is used. Python 3.9 or higher is required.
