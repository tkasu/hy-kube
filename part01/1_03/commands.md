# Ex 1.03

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.03
```

```
./docker_build_and_publish.sh ex1.03
```

## Create cluster

```
k3d cluster create -a 2
```

## Deployment

```
k3d cluster start
kubectl apply -f manifests/deployment.yaml
```

## Check logs

```
§ kubectl get pods
NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-mainapp-dep-5f79f66b56-4c8cc   1/1     Running   0          13s


§ kubectl logs hy-kube-mainapp-dep-5f79f66b56-4c8cc | tail

2020-08-29T17:13:50.722434900Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:13:55.687816600Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:00.688643Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:05.690609600Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:10.691099Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:15.692003100Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:20.692485200Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:25.657958700Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:30.658203Z: db584085-4678-46df-a288-3b63d2240733
2020-08-29T17:14:35.658448800Z: db584085-4678-46df-a288-3b63d2240733

```