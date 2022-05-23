# Ex 1.13

## Building pingpong application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.01
```

In folder pingpong/

```
./docker_build_and_publish.sh ex2.01
```


## Building mainapp application

```
git checkout ex2.01
```

In folder mainapp/

```
./docker_build_and_publish.sh ex2.01
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

$ kubectl apply -f manifests_global/

persistentvolume/hy-kube-pv created
```

## Deployment

### pingpong

In folder pingpong:

```
$ kubectl apply -f manifests/

deployment.apps/hy-kube-pingpong-dep created
ingress.extensions/hy-kube-pingpong-ingress created
service/hy-kube-pingpong-svc created
```

### mainapp

In folder mainapp/:

```
$ kubectl apply -f manifests/

deployment.apps/hy-kube-mainapp-dep created
ingress.extensions/hy-kube-mainapp-ingress configured
service/hy-kube-mainapp-svc created
```


## Testing

```bash
$ curl localhost:8081
2021-02-08T18:17:27.039695905Z 08d6091b-ebe5-4657-8633-c2d63669b559
Ping / Pongs: 0

$ curl localhost:8081/pingpong
pong 0

$ curl localhost:8081/pingpong
pong 1

$ curl localhost:8081 
2021-02-08T18:18:12.069029607Z 08d6091b-ebe5-4657-8633-c2d63669b559
Ping / Pongs: 2
```
