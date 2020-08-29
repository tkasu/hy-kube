#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build -t hy-kube-backend .
docker tag hy-kube-backend tkasu/hy-kube-backend:$1
docker push tkasu/hy-kube-backend:$1
