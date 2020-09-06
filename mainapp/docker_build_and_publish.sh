#!/bin/sh

if [ "$#" -ne 1 ]
then
  echo "Usage: requires positional parameter tag_name"
  exit 1
fi

docker build -t hy-kube-mainapp-reader -f Dockerfile.reader .
docker build -t hy-kube-mainapp-writer -f Dockerfile.writer .

docker tag hy-kube-mainapp-reader tkasu/hy-kube-mainapp-reader:$1
docker tag hy-kube-mainapp-writer tkasu/hy-kube-mainapp-writer:$1

docker push tkasu/hy-kube-mainapp-reader:$1
docker push tkasu/hy-kube-mainapp-writer:$1