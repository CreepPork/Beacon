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
            ['cargo build {}'.format('--release' if is_release else '')], shell=True)

        if build.returncode > 0:
            print(build.stderr)
            return build.returncode

    cwd = os.getcwd()
    target_dir = 'target/release' if is_release else 'target/debug'

    files_to_delete = glob.glob(os.path.join(cwd, 'extensions/') + 'beacon*')

    for to_delete in files_to_delete:
        os.remove(to_delete)

    if running_os == 'Windows':
        shutil.copyfile(
            os.path.join(cwd, target_dir, 'libbeacon.dll'),
            os.path.join(cwd, 'extensions/beacon_x64.dll')
        )
    else:
        shutil.copyfile(
            os.path.join(cwd, target_dir, 'libbeacon.so'),
            os.path.join(cwd, 'extensions/beacon.so')
        )


if __name__ == "__main__":
    sys.exit(main())
