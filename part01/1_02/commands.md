# Ex 1.02

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.02
```

After that use in the git root docker_build_and_publish.sh

```
./docker_build_and_publish.sh ex1.02
```

## Create cluster

```
k3d cluster create -a 2
```

## Deployment

```
k3d cluster start
kubectl create deployment hy-kube-backend-dep --image=tkasu/hy-kube-backend:ex1.02
```

## Check logs

```
§ kubectl get pods
NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-backend-dep-59fcb477c8-bvkpx   1/1     Running   0          16s

§ kubectl logs hy-kube-backend-dep-59fcb477c8-bvkpx | tail
    => address: 0.0.0.0
    => port: 8000
    => log: critical
    => workers: 12
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
Rocket has launched from http://0.0.0.0:8000
```