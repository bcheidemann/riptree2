#!/bin/bash
set -e

touch a.txt
mkdir b
touch b/a.txt
mkdir c
touch c/a.txt
mkdir c/b
touch c/b/a.txt
