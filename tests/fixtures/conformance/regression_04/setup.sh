#!/bin/bash
set -e

do_iter() {
    mkdir dir
    cd dir
    touch a.txt
    mkdir folder
    cd folder
    touch a.txt
}

do_iter
do_iter
do_iter
do_iter
do_iter
do_iter
do_iter
do_iter
