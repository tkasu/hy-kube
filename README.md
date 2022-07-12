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

Kubernetes manifests are tested GKE, see infra definition in gcp_resources/ for GKE versions.

For SQL backend, GCP Cloud SQL for Postgres is used for project to get a fully managed database.
For legacy reasons, pingpong is using hosted postgres container as database.

GCP infra can be deployed with:

```
make gcp-infra-up
```

And destroyed with:

```
make gcp-infra-down
```

For detailed instructions, see the latest section from part0N folders.
