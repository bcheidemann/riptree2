#!/bin/bash
set -e

mkdir a.include
touch a.include/a.include
touch a.include/b.txt
mkdir a.include/c
touch a.include/c/a.include
touch a.include/c/b.txt
mkdir b
touch b/a.include
touch b/b.txt
