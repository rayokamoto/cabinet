# Cabinet
Cross-platform file sorting system that sorts files based on their attributes, such as file type,
file name and date modified.

#### *Disclaimer: Note that features may be added, revised, or removed at any time as this is in active development*

---

In future, options to do things like renaming and deleting a bunch of files may be implemented,
though that is not a priority.

### Currently implemented:
* [x] Sort by file type
* [x] Sort by date modified
* [x] Sort by file name
* [x] Sort by file size
* [ ] Load commands from a config or task file

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
cab <command> [<options>] <path>
```

<h3 align="center">
    More in-depth information is available in the <a href="/docs/features.md">features documentation</a>.
</h3>

#### Commands
* `type` - Sort by file type (e.g. `.jpg`, `.png`, `.pdf`, etc.)
* `date` - Sort by date modified (before or after given date in `YYYY-MM-DD` format)
* `name` - Sort by name of file (by whether given pattern is in the name or not)
* `size` - Sort by size of file in KB (by whether file is smaller or larger than given size)

There are two options for sorting files: absolute path and path templates:
#### Using absolute paths
Use `-p` or `--path` to use absolute or relative paths. This option is used by default. Note that if the directory that you are sorting is in the same directory as `cab`, then you can just use relative paths. 
```
cab type -a "C:\Users\User\Downloads"
cab type -a "/home/User/Downloads"
```
No flags provided will default to absolute path:
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
* Documents
* Downloads
* Desktop
* Home directory (e.g. `C:\Users\User\` or `/home/User/`)
* Music (or Audio)
* Pictures
* Videos (or Movies)

## Testing
For testing, such as generating test files to sort, Python is used. Python 3.7+ is recommended.
