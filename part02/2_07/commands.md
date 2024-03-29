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
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

For disk space issues, you can try the following args when creating the cluster:
```
--k3s-agent-arg '--kubelet-arg=eviction-hard=imagefs.available<1%,nodefs.available<1%' --k3s-agent-arg '--kubelet-arg=eviction-minimum-reclaim=imagefs.available=1%,nodefs.available=1%'
```

For background for the --k3s-agent-args, see: https://k3d.io/faq/faq/#pods-evicted-due-to-lack-of-disk-space

## Creating the namespace

```
$ kubectl apply -f manifests_global/mainapp-namespace.yaml

namespace/mainapp created
```

## Deployment

## Init & deploy secrets

in pingpong/manifests/secrets


Init secret when setting up for the first time:

Create template for Secret (without saving it to version control):

```yml
# pingpong/manifests/secrets/postgres-pwd.yaml
apiVersion: v1
kind: Secret
metadata:
  name: postgres-password
  namespace: mainapp
data:
  PASSWORD: U3VjaFZlcnlCaWdBbmRJbXBvcnRhbnRTZWNyZXQK  # FIXME, remember to encode to base64
```

```bash
age-keygen -o key.txt

sops --encrypt \
       --age age162unsh9g6c94es64yqljgtxc9j4z395crnz7cvqyxhm3shs8qusqnqynr4 \   # FIXME WITH YOUR PUBLIC KEY
       --encrypted-regex '^(data)$' \
       postgres-pwd.yaml > postgres-pwd.enc.yaml
```

Deploy:

```
$ SOPS_AGE_KEY_FILE=$(pwd)/key.txt sops --decrypt postgres-pwd.enc.yaml | kubectl apply -f -

secret/postgres-password created
```

### pingpong

in pingpong/

```
$ kubectl apply -f manifests/

configmap/pingpong-config created
deployment.apps/pingpong-dep created
middleware.traefik.containo.us/strip-prefix created
ingress.networking.k8s.io/pingpong-ingress created
configmap/pingpong-postgres-config created
service/pingpong-db created
statefulset.apps/postgres-ss created
service/pingpong-svc created
```

## Testing

```
$ kubectl get pods --namespace mainapp

NAME                           READY   STATUS    RESTARTS      AGE
postgres-ss-0                  1/1     Running   0             86s
pingpong-dep-87c6f578c-hbz4j   1/1     Running   3 (36s ago)   86s

$ curl localhost:8081/pingpong
pong 0% 

$ curl localhost:8081/pingpong/pings
{"ping_id":"hy_kube_ping","ping_count":1}%

# Test that state still persist after deletes
$ kubectl delete pod pingpong-dep-87c6f578c-hbz4j --namespace mainapp
pod "pingpong-dep-87c6f578c-hbz4j" deleted

$ kubectl delete pod postgres-ss-0 --namespace mainapp
pod "postgres-ss-0" deleted

$ kubectl get pods --namespace mainapp
NAME                           READY   STATUS    RESTARTS   AGE
pingpong-dep-87c6f578c-ck2kr   1/1     Running   0          51s
postgres-ss-0                  1/1     Running   0          19s

$ curl localhost:8081/pingpong/pings
{"ping_id":"hy_kube_ping","ping_count":1}%
```