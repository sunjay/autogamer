#!/usr/bin/env python3

################################################################################
#
# Generates a JSON file that maps actions like idle, walk, jump, etc. to
# sprites in a spritesheet.
#
# Takes a JSON configuration file and a spritesheet image and generates the data
# file at the given path.
#
################################################################################

import argparse

DESCRIPTION = """Generates a JSON file that maps actions like idle, walk,
jump, etc. to sprites in a spritesheet.

Outputs a JSON file at the same path as the image with the file extension of
the image replaced with `.json`."""

def parse_args():
    parser = argparse.ArgumentParser(description=DESCRIPTION)
    parser.add_argument('config')
    parser.add_argument('spritesheets', nargs='+')
    return parser.parse_args()

def main():
    args = parse_args()
    print(args)

if __name__ == "__main__":
    main()
