# Ex 1.09

## Building pingpong application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.09
```

In folder pingpong/

```
./docker_build_and_publish.sh ex1.09
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2
```

## Deployment

In folder pingpong/:

Deploy app, service and ingress

```
§ k3d cluster start

§ kubectl apply -f manifests/

deployment.apps/hy-kube-pingpong-dep created
ingress.extensions/hy-kube-ingress created
service/hy-kube-pingpong-svc created
```

Test

```
$ curl localhost:8081/pingpong
pong 0%
$ curl localhost:8081/pingpong
pong 1%
$ curl localhost:8081/pingpong
pong 2%
$ curl localhost:8081/pingpong
pong 3%    
```

