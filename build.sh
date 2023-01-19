#!/bin/env bash

if [ $# -eq 0 ]
  then
    >&2 echo -e "Missing commands. Exiting.\n Help: ./build.sh cargo build_slim"
    exit 1
fi

# Check for either podman or docker
if hash podman 2>/dev/null
  then
    command=podman
  elif hash docker 2>/dev/null
    then
      command=docker
  else
    >&2 echo 'Either podman or docker need to be installed. Exiting.'
    exit 1
fi

# only works from puppet-ops directory, because it's mounted in the container
if [ "$0" != "./build.sh" ]
  then
    >&2 echo 'The script must be run from the git work directory via "./build.sh"!'
    exit 1
fi

# with option "clean", clean created container
if [ "$1" = "clean" ]
then
  $command image rm libnss-mapper:latest 2>/dev/null
  exit 0
fi

if [ "$1" = "shell" ]
then
  $command run -v "$(pwd)":/"$(basename "${PWD}")" -w "/$(basename "${PWD}")" -e "TERM=xterm-256color" -it --rm libnss-mapper /bin/bash -l
  exit 0
fi

# if there is no image yet, build it
if ! [[ "$($command image ls -qf 'reference=libnss-mapper:latest')" ]]
then
  $command build --tag libnss-mapper .
fi

$command run -v "$(pwd)":/"$(basename "${PWD}")" -w "/$(basename "${PWD}")" -e "TERM=xterm-256color" --rm libnss-mapper /bin/bash -l -c "$*"
