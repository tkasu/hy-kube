# Ex 2.07

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.07
```

## Building the applications


In folder /pingpong/

```
./docker_build_and_publish.sh ex2.07
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2 --k3s-agent-arg '--kubelet-arg=eviction-hard=imagefs.available<1%,nodefs.available<1%' --k3s-agent-arg '--kubelet-arg=eviction-minimum-reclaim=imagefs.available=1%,nodefs.available=1%'
```

For background for the --k3s-agent-args, see: https://k3d.io/faq/faq/#pods-evicted-due-to-lack-of-disk-space

## Init filesystem

```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
```

```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube_pg
```

## Install kubeseal

```bash
kubectl apply -f https://github.com/bitnami-labs/sealed-secrets/releases/download/v0.15.0/controller.yaml
```

## Creating the namespace

```
$ kubectl apply -f manifests_global/mainapp-namespace.yaml

namespace/mainapp created
```

## Deployment

### pingpong

```
$ kubectl apply -f pingpong/manifests/

deployment.apps/pingpong-dep created
ingress.extensions/pingpong-ingress created
configmap/pingpong-postgres-config created
persistentvolume/postgres-pv created
sealedsecret.bitnami.com/postgres-password created
service/pingpong-db created
statefulset.apps/postgres-ss 
```

Please note, that the sealed secret is not working in other machines than my local machines. If you want to get it working on your cluster, create a following yaml to folder manifests_temp:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: postgres-password
  namespace: mainapp
data:
  PASSWORD: cGluZ3BvbmcK
```

And update the sealed secret by running:

```bash
$ kubeseal -o yaml < pingpong/manifests_temp/postgres-pwd-secret.yaml > pingpong/manifests/postgres-pwd-sealedsecret.yaml
$ kubectl apply -f pingpong/manifests/postgres-pwd-sealedsecret.yaml
sealedsecret.bitnami.com/postgres-password created
```

## Testing

```
$ kubectl get pods --namespace mainapp
NAME                                    READY   STATUS        RESTARTS   AGE
hy-kube-pingpong-dep-5cffc455bf-z6j99   1/1     Running       0          7m3s<z
hy-kube-mainapp-dep-5f97b889b7-8pslp    2/2     Running       0          29s

$ curl localhost:8081
Hello
2021-03-09T12:31:17.274288090Z a00543af-3486-4403-b73e-723ec1df2390
Ping / Pongs: 0%
```