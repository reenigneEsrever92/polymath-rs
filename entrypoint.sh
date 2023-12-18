#!/bin/bash

export PATH=$PATH:/osxcross/target/bin
. $HOME/.cargo/env

exec "$@"

