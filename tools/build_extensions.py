#!/usr/bin/env python3
import sys
import subprocess
import os
import shutil
import platform
import glob


def is_production():
    return '--release' in sys.argv


def is_automated():
    return '--ci' in sys.argv


def main():
    is_release = is_production()
    is_ci = is_automated()

    print('Is release build? {}'.format(is_release))
    print('Is CI build? {}'.format(is_ci))

    running_os = platform.system()

    if running_os != 'Windows' and running_os != 'Linux':
        print('{} is not a supported OS.'.format(running_os))
        return 1

    root_dir = ".."
    if os.path.exists("addons"):
        root_dir = "."

    os.chdir(root_dir)

    if not is_ci:
        build = subprocess.run(
            ['cargo build --target i686-unknown-linux-gnu {}'.format('--release' if is_release else '')], shell=True)

        if build.returncode > 0:
            print(build.stderr)
            return build.returncode

    cwd = os.getcwd()
    env = 'release' if is_release else 'debug'

    files_to_delete = []

    if running_os == 'Windows':
        files_to_delete = glob.glob(cwd + os.path.sep + 'beacon*.dll')
    else:
        files_to_delete = glob.glob(cwd + os.path.sep + 'beacon*.so')

    for to_delete in files_to_delete:
        os.remove(to_delete)

    if running_os == 'Windows':
        shutil.copyfile(
            os.path.join(cwd, 'target', env, 'libbeacon.dll'),
            os.path.join(cwd, 'beacon_x64.dll')
        )
    else:
        shutil.copyfile(
            os.path.join(cwd, 'target/i686-unknown-linux-gnu',
                         env, 'libbeacon.so'),
            os.path.join(cwd, 'beacon.so')
        )


if __name__ == "__main__":
    sys.exit(main())
