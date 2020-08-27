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
NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-backend-dep-85c54d6ff5-pn4qj   1/1     Running   0          4m45s

§ kubectl logs hy-kube-backend-dep-85c54d6ff5-pn4qj | tail  
2020-08-27T17:39:02.666593474Z: 9fd5f51e-b0f2-4984-b5c1-befc191d1700
2020-08-27T17:39:07.666809937Z: a7165d71-fd6e-4dfb-bad4-e41906806bc5
2020-08-27T17:39:12.666988655Z: 8152e5b4-d52c-49e2-97ae-a270644b5118
2020-08-27T17:39:17.667394688Z: a80aeccc-3bdc-476f-840f-cce63c8997dd
2020-08-27T17:39:22.667611571Z: c163e096-a1cb-4e42-be7e-ff481bab5dea
2020-08-27T17:39:27.667813460Z: 4b12291d-16ae-4ce4-b428-724790356f77
2020-08-27T17:39:32.668010518Z: aceb57e6-fee6-4e38-80d6-205e772663a3
2020-08-27T17:39:37.668408488Z: 12217aac-4b75-4754-a92f-ff392d0cecca
2020-08-27T17:39:42.668698020Z: 54cc550d-d811-48a9-9fb8-3d78edc29a09
2020-08-27T17:39:47.669120740Z: 50436b2f-ba45-4d82-af63-c0e577ca052f
```