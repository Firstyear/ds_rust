#!/bin/sh
cat $1 | grep -E '^version.*' | awk '{print $3}'
