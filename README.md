# hy-kube

My exercises for University of Helsinki MOOC [DevOps with Kubernetes](https://devopswithkubernetes.com)

## Repository structure

NOTE! To get application version for some specific exercise, you can checkout this branch
with exercise specific tag, e.g. `git checkout ex2.10`.

### part0N folders

Commands and test for exercises

### mainapp (log output application)

Main / Log output application

### pingpong

Pingpong application

### project

Main project for the course, contains subfolders for backend and frontend.

## Requirements

Kubernetes manifests are tested with apple silicon based k3d & k3s versions:
k3d version v5.4.2
k3s version v1.23.6-k3s1 (default)

However, any hosting platform should be fine if the containers are built again. To build e.g. the backend for the project:

```
$ cd project/backedn
$ ./docker_build_and_publish.sh ex2.10
```

However, note that you probably need to change the Docker Hub repository specified in docker_build_and_publish.sh.
