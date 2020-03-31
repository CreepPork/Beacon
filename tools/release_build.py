import sys
import os
import platform
import shutil


def main():
    root_dir = ".."
    if os.path.exists("addons"):
        root_dir = "."

    os.chdir(root_dir)
    cwd = os.getcwd()

    shutil.move(
        os.path.join(cwd, "releases/@beacon.zip"),
        os.path.join(
            cwd, "releases/@beacon_{}.zip".format(platform.system().lower()))
    )


if __name__ == "__main__":
    sys.exit(main())
