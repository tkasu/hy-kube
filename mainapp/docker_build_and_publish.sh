#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build -t hy-kube-mainapp .
docker tag hy-kube-mainapp tkasu/hy-kube-mainapp:$1
docker push tkasu/hy-kube-mainapp:$1
