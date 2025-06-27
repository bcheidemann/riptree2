#!/bin/bash
set -e

mkdir a.include
touch a.include/a.txt
touch a.include/b.include
mkdir a.include/c.include
touch a.include/c.include/a.txt
touch a.include/c.include/b.include
mkdir a.include/c.include/c.include
touch a.include/c.include/c.include/a.txt
touch a.include/c.include/c.include/b.include
mkdir a.include/c.include/d
touch a.include/c.include/d/a.txt
touch a.include/c.include/d/b.include
