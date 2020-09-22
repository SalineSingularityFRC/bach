#!/bin/sh

javac src/*.java -d . || exit 1
java Bach
