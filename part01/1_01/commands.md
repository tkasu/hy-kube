# Ex 1.01

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.01
```

After that use in the git root docker_build_and_publish.sh

```
./docker_build_and_publish.sh ex1.01
```

## Create cluster

```
k3d cluster create -a 2
```

## Deployment

```
k3d cluster start
kubectl create deployment hy-kube-backend-dep --image=tkasu/hy-kube-backend:ex1.01
```

## Check logs


```
§ kubectl get pods
NAME                                   READY   STATUS        RESTARTS   AGE
hy-kube-backend-dep-85c54d6ff5-hpz84   1/1     Running       0          21s

§ kubectl logs hy-kube-backend-dep-85c54d6ff5-hpz84 | tail
2020-08-28T10:33:19.825497260Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:24.826146929Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:29.828027082Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:34.828188135Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:39.829457930Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:44.830153180Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:49.830393254Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:54.831429036Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:33:59.832774448Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
2020-08-28T10:34:04.833713826Z: 1a5b0926-b434-42f0-a453-6cac6453cdc7
```