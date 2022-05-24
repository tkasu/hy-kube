# Ex 2.06

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.06
```

## Building the applications


In folder /mainapp/

```
./docker_build_and_publish.sh ex2.06
```

In folder /pingpong/

```
./docker_build_and_publish.sh ex2.06
```

## Create cluster

```
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
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
$ kubectl apply -f manifests_global/mainapp-namespace.yaml

namespace/mainapp created
```

## Deployment

### mainapp

In folder mainapp:

```
$ kubectl apply -f manifests/

configmap/mainapp-configmap created
deployment.apps/hy-kube-mainapp-dep created
ingress.extensions/hy-kube-mainapp-ingress created
service/hy-kube-mainapp-svc created
```

### pingpong

In folder pingpong:

```
$ kubectl apply -f manifests/

deployment.apps/hy-kube-backend-dep created
ingress.extensions/hy-kube-backend-ingress created
persistentvolumeclaim/hy-kube-project-claim created
service/hy-kube-backend-svc created
```

## Testing

```
$ kubectl get pods --namespace mainapp
NAME                                    READY   STATUS        RESTARTS   AGE
hy-kube-pingpong-dep-5cffc455bf-z6j99   1/1     Running       0          7m3s
hy-kube-mainapp-dep-5f97b889b7-8pslp    2/2     Running       0          29s

$ curl localhost:8081
Hello
2021-03-09T12:31:17.274288090Z a00543af-3486-4403-b73e-723ec1df2390
Ping / Pongs: 0%
```