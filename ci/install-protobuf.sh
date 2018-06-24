#!/bin/sh

set -ex

curl -sL https://github.com/google/protobuf/releases/download/v3.5.1/protobuf-cpp-3.5.1.tar.gz | tar zx

cd protobuf-3.5.1

./configure --prefix=/home/travis && make -j2 && make install

test -x /home/travis/bin/protoc
