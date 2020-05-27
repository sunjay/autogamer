#!/usr/bin/env python3

import os
import sys
import argparse
import subprocess

from shutil import copy2

class App:
    def __init__(self, *, libname, pydir):
        self.libname = libname
        self.pydir = pydir

class Linux:
    @staticmethod
    def dylib(libname):
        return "lib{}.so".format(libname)

    @staticmethod
    def nativemod(libname):
        return "{}.so".format(libname)

class MacOS:
    @staticmethod
    def dylib(libname):
        return "lib{}.dylib".format(libname)

    @staticmethod
    def nativemod(libname):
        return "{}.so".format(libname)

class Windows:
    @staticmethod
    def dylib(libname):
        return "lib{}.dll".format(libname)

    @staticmethod
    def nativemod(libname):
        return "{}.pyd".format(libname)

def get_platform(target=None):
    platform = target or sys.platform
    if "linux" in platform:
        return Linux()
    elif "darwin" in platform:
        return MacOS()
    elif "win" in platform or "cygwin" in platform:
        return Windows()

    if target is None:
        raise ValueError("x.py is running on unsupported platform")
    else:
        raise ValueError("x.py ran with unsupported target: {}".format(target))

def run_command(cmd):
    print(" ".join(cmd))
    subprocess.run(cmd, check=True)

def copy(src, dst):
    print("cp '{}' '{}'".format(src, dst))
    copy2(src, dst)

def parse_args():
    parser = argparse.ArgumentParser(description="Process some integers.")
    subcommands = parser.add_subparsers(dest="subcommand")

    build = subcommands.add_parser("build")
    build.add_argument("--release", action="store_true")
    build.add_argument("--target", metavar="TRIPLE")

    args, unknownargs = parser.parse_known_args()
    return parser, args, unknownargs

def build(app, args, build_args):
    if args.release:
        mode = "release"
        # Insert the argument back into the command arguments
        build_args.insert(0, "--release")
    else:
        mode = "debug"

    # Find path to compilation artifacts
    target_path = "target"
    if args.target is not None:
        target_path = os.path.join(target_path, args.target)
        # Insert the argument back into the command arguments
        build_args.insert(0, args.target)
        build_args.insert(0, "--target")
    target_path = os.path.join(target_path, mode)

    # Compile the program to generate compilation artifacts
    run_command(["cargo", "build", "--all", *build_args])

    # Figure out the platform
    platform = get_platform(args.target)

    dylib_path = os.path.join(target_path, platform.dylib(app.libname))
    nativemod_path = os.path.join(app.pydir, platform.nativemod(app.libname))
    copy(dylib_path, nativemod_path)

def main():
    parser, args, unknownargs = parse_args()

    # Set the current directory so all paths and commands are relative to the
    # right directory
    os.chdir(os.path.dirname(__file__))

    app = App(
        libname="autogamer_bindings",
        pydir="pyautogamer",
    )

    if args.subcommand == "build":
        build(app, args, unknownargs)
    else:
        parser.print_usage()
        sys.exit(1)

if __name__ == "__main__":
    main()
