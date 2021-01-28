# Ex 1.12

## Building mainapp application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.12
```

In folder project/backend/

```
./docker_build_and_publish.sh ex1.12
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

$ kubectl apply -f manifests_global/

persistentvolume/hy-kube-pv created
```

## Deployment

In folder project/backend/:

```

$ kubectl apply -f manifests/

deployment.apps/hy-kube-backend-dep unchanged
ingress.extensions/hy-kube-ingress configured
persistentvolumeclaim/hy-kube-project-claim configured
service/hy-kube-backend-svc unchanged
```

```bash
$ curl localhost:8081

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>HY-Kubernetes CRUD app!</title>
</head>
<body>
    <h1>hy-kube crud-app!</h1>
    <img src="daily_pic.jpg" alt="daily pic!">
</body>
</html>%           
```

Logs from the initial deployment:

```
Using config: Config { image_update_interval: 86400, image_cache_path: "/usr/app/files/img.jpg", image_state_path: "/usr/app/files/img_state.json" }
WARNING!, Failed to copy image from cache, Os { code: 2, kind: NotFound, message: "No such file or directory" }
Re-fetching image instead.
WARNING: Could not read existing image state: Os { code: 2, kind: NotFound, message: "No such file or directory" }
Updating image!
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
Rocket has launched from http://0.0.0.0:3000
Postponing image update until 2021-01-29 12:39:40 UTC
```

Restarting pods:

```bash
$ kubectl rollout restart deployment hy-kube-backend-dep
deployment.apps/hy-kube-backend-dep restarted
```

Logs after restart (using the image from cache instead of reloading it):

```
Using config: Config { image_update_interval: 86400, image_cache_path: "/usr/app/files/img.jpg", image_state_path: "/usr/app/files/img_state.json" }
Postponing image update until 2021-01-29 12:39:40 UTC
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
Rocket has launched from http://0.0.0.0:3000
```



