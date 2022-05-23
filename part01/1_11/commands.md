# Ex 1.11

## Notes

```bash
k # is alias for kubectl 
```

## Building mainapp application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.11
```

In folder mainapp/

```
./docker_build_and_publish.sh ex1.11
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

$ k apply -f manifests_global/

persistentvolume/hy-kube-pv created
```

## Deployment

In folder mainapp/:

```

§ k apply -f manifests/with_pingpong/

deployment.apps/hy-kube-mainapp-dep created
ingress.extensions/hy-kube-ingress created
persistentvolumeclaim/hy-kube-mainapp-pingpong-claim created
service/hy-kube-mainapp-svc created
```

Test

```bash
$ k get pods

NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-mainapp-dep-59758f9d4b-qbrcp   3/3     Running   0          3m5s

$ k describe ingress

Name:             hy-kube-ingress
Labels:           <none>
Namespace:        default
Address:          172.18.0.3,172.18.0.4,172.18.0.5
Ingress Class:    <none>
Default backend:  <default>
Rules:
  Host        Path  Backends
  ----        ----  --------
  *
              /           hy-kube-mainapp-svc:2345 (10.42.1.5:3000)
              /pingpong   hy-kube-mainapp-svc:3567 (10.42.1.5:4000)
Annotations:  traefik.ingress.kubernetes.io/router.middlewares: default-strip-prefix@kubernetescrd
Events:       <none>

````

```bash
$ curl localhost:8081 
2020-09-07T18:31:19.219171724Z b19a543f-87ce-4c2e-a11b-8d0918be6773
Ping / Pongs: 0%

$ curl localhost:8081/pingpong
pong 0

$ curl localhost:8081/pingpong
pong 1%

$ curl localhost:8081         
2020-09-07T18:31:59.222298082Z b19a543f-87ce-4c2e-a11b-8d0918be6773
Ping / Pongs: 2
```

