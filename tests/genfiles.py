import glob
import os
import shutil
import sys
import time
import threading

MIN_FILES_TO_LOOP_ONCE = 1000

def remove_files(files):
    for f in files:
        if os.path.isfile(f):
            os.remove(f)
        elif os.path.isdir(f):
            #os.rmdir(f) # Only works with empty dirs
            shutil.rmtree(f)
        else:
            print("Object was not a file or directory.")
            print(f, "---")

def clean(path):
    files = glob.glob(f"{path}/*", recursive=True)
    file_count = len(files)
    loop_count = os.cpu_count()
    if file_count < MIN_FILES_TO_LOOP_ONCE:
        loop_count = 1

    leftover_files: int = file_count % loop_count
    files_per_thread: int = file_count // loop_count

    split_files = []
    start = time.perf_counter()
    for i in range(loop_count):
        if i == loop_count-1:
            split_files.append(files[i*files_per_thread::])
        else:
            split_files.append(files[i*files_per_thread:(i+1)*files_per_thread])

    threads = []
    for i in range(loop_count):
        t = threading.Thread(target=remove_files, args=(split_files[i],))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    end = time.perf_counter()

    print(f"All files and/or directories have been removed. Process took {round(end-start, 2)}s")

def gen_files(path, file_type, start_idx, end_idx):
    for j in range(start_idx, end_idx+1):
        open(f"{path}/File_{j}.{file_type}", "w")


def main(argv, argc):
    # The directory where this file is run from
    cwd = os.getcwd()
    path = cwd + "/test_dir"
    FILE_COUNT = 1_000

    if argc == 1:
        if not os.path.isdir(path):
            os.mkdir(path)

        # TODO: Test file types with invalid unicode
        file_types = [
            "abc",
            "def",
            "ghi",
            "txt",
            "😀😀",
            "123",
            "md"
        ]

        total_files = FILE_COUNT * len(file_types)
        loop_count = os.cpu_count()
        if total_files < MIN_FILES_TO_LOOP_ONCE:
            loop_count = 1

        print(f"{loop_count} threads will be used to create the files")

        start = time.perf_counter()
        term_width = os.get_terminal_size().columns
        progbar_width = term_width - 28

        for idx, file_type in enumerate(file_types):
            threads = []

            leftover_files = FILE_COUNT % loop_count
            files_per_thread = FILE_COUNT // loop_count

            for i in range(loop_count):
                f = files_per_thread
                if i == loop_count-1:
                    f = files_per_thread + leftover_files
                start_idx = i*f+1
                end_idx = (i+1)*f
                t = threading.Thread(target=gen_files, args=(path, file_type, start_idx, end_idx))
                t.start()
                threads.append(t)

            for t in threads:
                t.join()

            progress = int((idx+1)*FILE_COUNT/total_files * progbar_width)
            done = int(progress)*"#"
            not_done = int((progbar_width-progress))*"-"
            percent = int(progress/progbar_width * 100)

            now = time.perf_counter()
            elapsed = f"{round(now-start, 2)}s"
            print(f"Progress: {elapsed:>8} [{done}{not_done}] {percent}%", end="\r")

        print("\n")
        print(f"{FILE_COUNT * len(file_types)} files have been generated.")

    elif argv[1] == "clean":
        clean(path)
    else:
        pass


if __name__ == "__main__":
    argv = sys.argv
    argc = len(argv)
    main(argv, argc)
