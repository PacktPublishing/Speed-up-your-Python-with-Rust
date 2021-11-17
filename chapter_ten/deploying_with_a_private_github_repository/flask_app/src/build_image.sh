#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

git clone https://github.com/maxwellflitton/flitton-fib-rs.git
cd flitton-fib-rs
git checkout $1
cd ..
rm -rf ./flitton-fib-rs/.git

docker build . --no-cache -t flask-fib
rm -rf ./flitton-fib-rs

RUN pip install ./flitton-fib-rs
RUN rm -rf ./flitton-fib-rs
