#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build -t hy-kube-pingpong .
docker tag hy-kube-pingpong tkasu/hy-kube-pingpong:$1
docker push tkasu/hy-kube-pingpong:$1
