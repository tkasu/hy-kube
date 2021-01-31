#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build --no-cache -t hy-kube-frontend -f Dockerfile.kube .
docker tag hy-kube-frontend tkasu/hy-kube-frontend:$1
docker push tkasu/hy-kube-frontend:$1
