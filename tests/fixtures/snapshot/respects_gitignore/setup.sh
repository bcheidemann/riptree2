#!/bin/bash
set -e

echo "/e
/a.txt
*.ignore
" > .gitignore
touch a.txt
touch b.txt
touch c.ignore
mkdir d
echo "!*.not.ignore
ignored.txt
" > d/.gitignore
touch d/a.txt
touch d/b.not.ignore
touch d/ignored.txt
mkdir e
touch e/a.txt
mkdir f
touch f/a.txt
