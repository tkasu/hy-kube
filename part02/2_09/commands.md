# Ex 2.09

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.09
```

## Building the applications


In folder project/backend/

```
./docker_build_and_publish.sh ex2.09
```

In folder project/frontend/

```
./docker_build_and_publish.sh ex2.09
```

## Create cluster

```
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

## Init filesystem

```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
```

## Global services:

In root folder:

```bash
§ k3d cluster start

$ kubectl apply -f manifests_global/persistentvolume.yaml

persistentvolume/hy-kube-pv created
```

## Creating the namespace

```
kubectl apply -f manifests_global/hy-kube-project-namespace.yaml

namespace/hy-kube-project created
```

## Deployment

## Init & deploy secrets

in backend/manifests/secrets


Init secret when setting up for the first time:

Create template for Secret (without saving it to version control):

```yml
# project/backend/manifests/secrets/postgres-pwd.yaml
apiVersion: v1
kind: Secret
metadata:
  name: postgres-password
  namespace: hy-kube-project
data:
  PASSWORD: QW5vdGhlclZlcnlCaWdBbmRJbXBvcnRhbnRTZWNyZXQ= # FIXME, remember to encode to base64
```

```bash
age-keygen -o key.txt

sops --encrypt \
       --age age1k0upvtn0gwftpep5kxq47xztxj7ulmfhk6t9ha82sd6r5jrjsegsdr0wua \  # FIXME WITH YOUR PUBLIC KEY
       --encrypted-regex '^(data)$' \
       postgres-pwd.yaml > postgres-pwd.enc.yaml
```

Deploy:

```
$ SOPS_AGE_KEY_FILE=$(pwd)/key.txt sops --decrypt postgres-pwd.enc.yaml | kubectl apply -f -

secret/postgres-password created
```

### backend

in project/backend

```
$ kubectl apply -f manifests/

cronjob.batch/randomtask created
configmap/backend-config created
deployment.apps/backend-dep created
middleware.traefik.containo.us/strip-prefix created
ingress.networking.k8s.io/backend-ingress created
persistentvolumeclaim/project-claim created
configmap/project-db-config created
service/project-db-svc created
statefulset.apps/postgres-ss created
service/backend-svc created
```

### frontend

in project/frontend

```
$ kubectl apply -f manifests/

deployment.apps/frontend-dep created
ingress.networking.k8s.io/frontend-ingress created
service/frontend-svc created
```

## Testing

For testing, daily-todojob.yaml schedule is changed to "* * * * *":

```
$ kubectl get pods --namespace hy-kube-project

NAME                            READY   STATUS      RESTARTS   AGE
postgres-ss-0                   1/1     Running     0          8m35s
frontend-dep-8544647698-dptl6   1/1     Running     0          8m27s
backend-dep-65d9dc5cdb-dt4n9    1/1     Running     0          8m36s
randomtask-27577050--1-26kmd    0/1     Completed   0          3m4s
randomtask-27577051--1-scs72    0/1     Completed   0          2m4s
randomtask-27577052--1-f6lsp    0/1     Completed   0          64s
randomtask-27577053--1-ppg2w    1/1     Running     0          4s
```

```
$ curl localhost:8081/api/todos

{"todos":[{"task":"Read: https://en.wikipedia.org/wiki/Roque_J%C3%BAnior"},{"task":"Read: https://en.wikipedia.org/wiki/Family_Name_(film)"},{"task":"Read: https://en.wikipedia.org/wiki/Tuman_Leghari"},{"task":"Read: https://en.wikipedia.org/wiki/Korol_i_Shut_(album)"}]}% 
```
