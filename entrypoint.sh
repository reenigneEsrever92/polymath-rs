#!/bin/bash

echo $HOME/.profile
source $HOME/.profile

exec "$@"

