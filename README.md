# Cabinet
Cross-platform file sorting system

### Currently implemented:
* [x] File type
* [ ] Date modified
* [ ] File name

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
#### Commands
* `type` - Sort by file type (e.g. `.jpg`, `.png`, `.pdf`, etc.)
* `name` - Sort by name of file (Not yet implemented)
* `date` - Sort by date modified (Not yet implemented)

There are two options for sorting files: absolute path and path templates:
#### Using absolute paths:
Use `-p` or `--path` to use absolute paths
```
cab type -p "C:\Users\User\Downloads"
cab type -p "/home/User/Downloads"
```
#### Using templates
With templates, you can sort directories quickly. Invoke using the `-t` or `--template` option:
```
cab type -t downloads
```
Currently, the following templates are available:
* Desktop
* Documents
* Downloads
* Home directory (e.g. `C:\Users\User\` or `/home/User/`)


## Testing
For testing, such as generating test files to sort, Python is used. Python 3.7+ is recommended.
