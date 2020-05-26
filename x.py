#!/usr/bin/env python3

import os
import sys
import subprocess

from shutil import copy2

LIBNAME = "autogamer_bindings"

if sys.platform.startswith("linux"):
    dylib = "lib{}.so".format(LIBNAME)
    nativemod = "{}.so".format(LIBNAME)
elif sys.platform.startswith("win") or sys.platform.startswith("cygwin"):
    dylib = "lib{}.dll".format(LIBNAME)
    nativemod = "{}.pyd".format(LIBNAME)
elif sys.platform.startswith("darwin"):
    dylib = "lib{}.dylib".format(LIBNAME)
    nativemod = "{}.so".format(LIBNAME)
else:
    raise ValueError("x.py is running on unsupported platform")

def run_command(cmd):
    print(' '.join(cmd))
    subprocess.run(cmd, check=True)

def copy(src, dst):
    print("cp '{}' '{}'".format(src, dst))
    copy2(src, dst)

os.chdir(os.path.dirname(__file__))

extra_args = sys.argv[1:]
run_command(["cargo", "build", "--all", *extra_args])

mode = "release" if "--release" in extra_args else "debug"
dylib_path = os.path.join("target", mode, dylib)
nativemod_path = os.path.join("pyautogamer", nativemod)
copy(dylib_path, nativemod_path)
