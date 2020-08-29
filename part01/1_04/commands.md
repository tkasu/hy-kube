# Ex 1.04

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.04
```

```
./docker_build_and_publish.sh ex1.04
```

## Create cluster

```
k3d cluster create -a 2
```

## Deployment

In folder project/backend/:

```
k3d cluster start
kubectl apply -f manifests/deployment.yaml
```

## Check logs

```
§ kubectl get pods
NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-backend-dep-6dd6c64b48-lqkjc   1/1     Running   0          76s

§ kubectl logs hy-kube-backend-dep-6dd6c64b48-lqkjc | tail

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