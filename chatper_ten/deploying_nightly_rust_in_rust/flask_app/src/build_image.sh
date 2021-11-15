#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

git clone https://github.com/maxwellflitton/flitton-fib-rs.git
git clone https://github.com/maxwellflitton/rust-db-cloning.git

rm -rf ./flitton-fib-rs/.git
rm -rf ./rust-db-cloning/.git

docker build . --no-cache -t flask-fib

rm -rf ./flitton-fib-rs
rm -rf ./rust-db-cloning

RUN pip install ./flitton-fib-rs
RUN rm -rf ./flitton-fib-rs
