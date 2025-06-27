#!/bin/bash
set -e

mkdir a.ignore
touch a.ignore/a.txt
mkdir b
touch b/a.txt
mkdir c
touch c/a.ignore
touch d.txt
touch e.ignore
