#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build -t hy-kube-backend -f Dockerfile.server .
docker build -t hy-kube-backend-randomtask -f Dockerfile.randomtask .

docker tag hy-kube-backend tkasu/hy-kube-backend:$1
docker tag hy-kube-backend-randomtask tkasu/hy-kube-backend-randomtask:$1

docker push tkasu/hy-kube-backend:$1
docker push tkasu/hy-kube-backend-randomtask:$1
