# Ex 1.08

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.08
```

In folder project/backend/

```
./docker_build_and_publish.sh ex1.08
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2
```

## Deployment

In folder project/backend/:

Deploy app, service and ingress

```
§ k3d cluster start

§ kubectl apply -f manifests/

deployment.apps/hy-kube-backend-dep created
ingress.extensions/hy-kube-ingress created
service/hy-kube-backend-svc created
```

Test

```
§ curl localhost:8081/hello/ingress

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>HY-Kubernetes CRUD app!</title>
</head>
<body>
    <h1>Hello ingress!</h1>
</body>
</html>%  
```

