#!/bin/bash

script_dir=$(dirname $(readlink -f $0))

git config --local core.autocrlf false
git config --local commit.template "$script_dir/.git-commit-template"
