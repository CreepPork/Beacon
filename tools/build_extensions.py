#!/usr/bin/env python3
import sys
import subprocess
import os
import shutil
import platform
import glob


def main():
    is_release = '--release' in sys.argv
    is_ci = '--ci' in sys.argv

    print('Is release build? {}'.format(is_release))
    print('Is CI build? {}'.format(is_ci))

    running_os = platform.system()

    if running_os != 'Windows' and running_os != 'Linux':
        print('{} is not a supported OS.'.format(running_os))
        return 1

    root_dir = '..'
    if os.path.exists('addons'):
        root_dir = '.'

    os.chdir(root_dir)

    if not is_ci:
        commands = [shutil.which('cargo'), 'build']

        if running_os == 'Linux':
            commands.append('--target i686-unknown-linux-gnu')

        if is_release:
            commands.append('--release')

        build = subprocess.run(commands, shell=True)

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


if __name__ == '__main__':
    sys.exit(main())
