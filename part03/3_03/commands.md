# Ex 3.03

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.03
```

## Building the applications

Push the tag the github with 'ex'-prefix, e.g. 'ex3.03' and the Github Action will publish the project to Docker Hub repository: 'tkasu/hy-kube-*'

## Create cluster, other needed GCP resources and sync kubectl creds

```
make gcp-infra-preq gcp-infra-up gcp-sync-kubectl-creds
```

## Deployment

## Init secrets

in project/backend/manifests/secrets


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

## Deploy manifests

```
make apply-project-kube
```

## Testing

```bash
$ kubectl get ingress --namespace hy-kube-project
NAME              CLASS    HOSTS   ADDRESS          PORTS   AGE
backend-ingress   <none>   *       34.117.104.209   80      6m12s

$ curl 34.117.104.209
<!doctype html><html lang="en"><head><meta charset="utf-8"/><link rel="icon" href="/favicon.ico"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content="#000000"/><meta name="description" content="Web site created using create-react-app"/><link rel="apple-touch-icon" href="/logo192.png"/><title>hy-kube app!</title>...

$ curl 34.117.104.209/api/todos
{"todos":[]}% 

$ curl -X POST 34.117.104.209/api/todo \
   -H 'Content-Type: application/json' \
   -d '{"task":"Hello GKE!"}'
{"task":"Hello GKE!"}%    

$ curl 34.117.104.209/api/todos
{"todos":[{"task":"Hello GKE!"}]}%  
```

## Destroy cluster & other needed GCP resources

```
make gcp-infra-down
```
