# Ex 2.03

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.03
```

## Building the mainapp and pingpong


In folder mainapp/

```
./docker_build_and_publish.sh ex2.03
```

In folder project/frontend/

```
./docker_build_and_publish.sh ex2.03
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2
```

## Init filesystem

```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
```

## Global services:

In root folder:

```bash
§ k3d cluster start

$ kubectl apply -f manifests_global/persistentvolume.yaml

persistentvolume/hy-kube-pv created
```

## Creating the namespace

```
kubectl apply -f  manifests_global/mainapp-namespace.yaml

namespace/mainapp created
```

## Deployment

### mainapp

In folder mainapp


```
kubectl apply -f manifests/

deployment.apps/hy-kube-mainapp-dep created
ingress.extensions/hy-kube-mainapp-ingress created
service/hy-kube-mainapp-svc created
```

### pingpong

In folder pingpong


```
kubectl apply -f manifests/

deployment.apps/hy-kube-mainapp-dep created
ingress.extensions/hy-kube-mainapp-ingress created
service/hy-kube-mainapp-svc created
```

## Testing

```
$ kubectl get deployments.apps 
No resources found in default namespace.

$ kubectl get deployments.apps --namespace mainapp
NAME                   READY   UP-TO-DATE   AVAILABLE   AGE
hy-kube-mainapp-dep    1/1     1            1           89s
hy-kube-pingpong-dep   1/1     1            1           52s
```
