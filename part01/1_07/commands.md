# Ex 1.07

## Building main application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.07
```

In folder mainapp/

```
./docker_build_and_publish.sh ex1.07
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2
```

## Deployment

In folder mainapp/:

Deploy app

```
§ k3d cluster start

§ kubectl apply -f manifests/deployment.yaml
deployment.apps/hy-kube-mainapp-dep created
```

Deploy service

```
$ kubectl apply -f manifests/service.yaml 
service/hy-kube-mainapp-svc created
```

Deploy ingress

```
kubectl apply -f manifests/ingress.yaml 
ingress.extensions/hy-kube-ingress created
```

Test

```
§ kubectl get pods
NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-mainapp-dep-744f85cf4b-kxrw7   1/1     Running   0          77s

§ kubectl logs hy-kube-mainapp-dep-744f85cf4b-kxrw7| tail
2020-08-30T17:42:28.528922800Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:33.529138200Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:38.529416200Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:43.529706Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:48.530165900Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:53.530491200Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:42:58.496118600Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:43:03.496369700Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:43:08.496659200Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
2020-08-30T17:43:13.496846200Z: 2458e63a-6dce-49fa-93ea-1e653261b66e
```

```
§ curl localhost:8081
2458e63a-6dce-49fa-93ea-1e653261b66e% 
```

