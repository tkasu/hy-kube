# Ex 1.05

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.05
```

```
./docker_build_and_publish.sh ex1.05
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
$ kubectl get pods
NAME                                   READY   STATUS        RESTARTS   AGE
hy-kube-backend-dep-6444fdd46b-tx4wn   1/1     Running       0          13s


§ kubectl logs hy-kube-backend-dep-6444fdd46b-tx4wn 
Configured for production.
    => address: 0.0.0.0
    => port: 3000
    => log: critical
    => workers: 12
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
    => [extra] template_dir: "templates/"
Rocket has launched from http://0.0.0.0:3000


```

## Set port forwarding

Set port forwarding:

```
§ kubectl port-forward hy-kube-backend-dep-6444fdd46b-tx4wn 3003:3000

Forwarding from 127.0.0.1:3003 -> 3000
Forwarding from [::1]:3003 -> 3000
```

Test:

```
$ curl localhost:3003

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>HY-Kubernetes CRUD app!</title>
</head>
<body>
    <h1>Hello Kube!</h1>
</body>
</html>%  

$ curl localhost:3003/hello/Tomi

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>HY-Kubernetes CRUD app!</title>
</head>
<body>
    <h1>Hello Tomi!</h1>
</body>
</html>% 
```