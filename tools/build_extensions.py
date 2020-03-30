import sys
import subprocess
import os
import shutil
import platform


def main():
    running_os = platform.system()

    if running_os != 'Windows' and running_os != 'Linux':
        print('{} is not a supported OS.'.format(running_os))
        return 1

    root_dir = ".."
    if os.path.exists("addons"):
        root_dir = "."

    os.chdir(root_dir)

    build = subprocess.run(['cargo build'], shell=True)

    if build.returncode > 0:
        print(build.stderr)
        return build.returncode

    cwd = os.getcwd()

    if running_os == 'Windows':
        shutil.move(
            os.path.join(cwd, 'target/debug/libbeacon.dll'),
            os.path.join(cwd, 'extensions/beacon_x64.dll')
        )
    else:
        shutil.move(
            os.path.join(cwd, 'target/debug/libbeacon.so'),
            os.path.join(cwd, 'extensions/beacon.so')
        )


if __name__ == "__main__":
    sys.exit(main())
