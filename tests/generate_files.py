import glob
import os
import shutil
import sys


def clean(path):
    files = glob.glob(f"{path}/*", recursive=True)
    for f in files:
        if os.path.isfile(f):
            os.remove(f)
        elif os.path.isdir(f):
            #os.rmdir(f) # Only works with empty dirs
            shutil.rmtree(f)
        else:
            print("Object was not a file or directory.")
    
    print("All files and/or directories have been removed.")


def main(argv, argc):
    #home_path = os.path.expanduser('~')
    # The directory where this file is run from
    cwd = os.getcwd()
    path = cwd + "/test_dir"
    FILE_COUNT = 50
    
    if argc == 0:
        if not os.path.isdir(path):
            os.mkdir(path)
        
        file_types = [
            "abc",
            "def",
            "ghi",
            "txt",
            "😀😀",
            "123",
            ""
        ]

        for i in file_types:
            for j in range(FILE_COUNT):
                open(f"{path}/File_{j}.{i}", "w")
        
        print(f"{FILE_COUNT * len(file_types)} files have been generated.")

    elif argv[0] == "clean":
        clean(path)
    else:
        pass
        


if __name__ == "__main__":
    argv = sys.argv[1:]
    argc = len(argv)
    main(argv, argc)
