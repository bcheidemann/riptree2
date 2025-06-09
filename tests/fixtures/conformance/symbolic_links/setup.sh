#!/bin/bash
set -e

mkdir current_directory
mkdir current_directory/root
touch a.txt
mkdir b
touch b/a.txt
ln -s ../a.txt current_directory/root/a.txt.link
ln -s ../b current_directory/root/b.link
touch current_directory/root/c.txt
mkdir current_directory/root/d
touch current_directory/root/d/a.txt
ln -s ./root/c.txt current_directory/root/c.txt.link
ln -s ./root/d current_directory/root/d.link
