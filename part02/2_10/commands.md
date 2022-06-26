# Ex 2.10

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex2.10
```

## Building the applications


In folder project/backend/

```
./docker_build_and_publish.sh ex2.10
```

In folder project/frontend/

```
./docker_build_and_publish.sh ex2.04
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

Test initial todo state is empty:
```bash
$ curl localhost:8081/api/todos
{"todos":[]}%
```

Test can add a new todo:
```bash
$ curl -X POST localhost:8081/api/todo \
   -H 'Content-Type: application/json' \
   -d '{"task":"Hello from curl!"}'
{"task":"Hello from curl!"}%

$ curl localhost:8081/api/todos
{"todos":[{"task":"Hello from curl!"}]}% 
```

Test too long todo is not added
```bash
$ LONG_TODO_STR=$(python3 -c 'print("x" * 150)')
$ curl -X POST localhost:8081/api/todo \
   -H 'Content-Type: application/json' \
   -d "{\"task\":\"$LONG_TODO_STR\"}"
Todo's length: 150 is over the limit 140.% 

$ curl localhost:8081/api/todos
{"todos":[{"task":"Hello from curl!"}]}% 
```

Check logs:
```bash
$ kubectl get pods --namespace hy-kube-project
NAME                            READY   STATUS    RESTARTS        AGE
frontend-dep-85bfc5b9c6-z8n25   1/1     Running   0               3m46s
postgres-ss-0                   1/1     Running   0               3m51s
backend-dep-6dd76d9986-trkgl    1/1     Running   2 (3m30s ago)   3m51s

$ kubectl logs backend-dep-6dd76d9986-trkgl --namespace hy-kube-project
Rocket has launched from http://0.0.0.0:3000
Postponing image update until 2022-06-27 17:10:45 UTC
GET /todos:
   >> Matched: (todos) GET /todos
"{\"details_type\":\"request\",\"method\":\"GET\",\"params\":null,\"source_ip\":\"10.42.0.0\",\"uri\":\"/todos\"}"
   >> Outcome: Success
   >> Response succeeded.
POST /todo application/json:
   >> Matched: (new_todo) POST /todo application/json
"{\"details_type\":\"request\",\"method\":\"POST\",\"params\":{\"task\":\"Hello from curl!\"},\"source_ip\":\"10.42.2.0\",\"uri\":\"/todo\"}"
   >> Outcome: Success
   >> Response succeeded.
GET /todos:
   >> Matched: (todos) GET /todos
"{\"details_type\":\"request\",\"method\":\"GET\",\"params\":null,\"source_ip\":\"10.42.0.0\",\"uri\":\"/todos\"}"
   >> Outcome: Success
   >> Response succeeded.
POST /todo application/json:
   >> Matched: (new_todo) POST /todo application/json
"{\"details_type\":\"request\",\"method\":\"POST\",\"params\":{\"task\":\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"},\"source_ip\":\"10.42.1.1\",\"uri\":\"/todo\"}"
"{\"detail_type\":\"error\",\"err\":{\"BadRequest\":\"Todo's length: 150 is over the limit 140.\"}}"
   >> Outcome: Success
   >> Response succeeded.
GET /todos:
   >> Matched: (todos) GET /todos
"{\"details_type\":\"request\",\"method\":\"GET\",\"params\":null,\"source_ip\":\"10.42.2.0\",\"uri\":\"/todos\"}"
   >> Outcome: Success
   >> Response succeeded.
```
