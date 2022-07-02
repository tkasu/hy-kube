# Ex 3.03

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.03
```

## Building the applications

### Create cluster, other needed GCP resources and sync kubectl creds

```
make gcp-infra-preq gcp-infra-up gcp-sync-kubectl-creds
```

### Init secrets

Init (or update) the secret when setting up for the first time:

Create template for Secret (without saving it to version control) to
project/backend/manifests/secrets:

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

Generate a new key:
```bash
age-keygen -o project/backend/manifests/secrets/key.txt
```

Update public key in .sops.yaml:
```yml
# project/backend/manifests/secrets/.sops.yaml
creation_rules:
  - encrypted_regex: "^(data)$"
    age: "age1k0upvtn0gwftpep5kxq47xztxj7ulmfhk6t9ha82sd6r5jrjsegsdr0wua" # FIXME
```

Encrypt the key:
```
make encrypt-project-secrets
```

Commit the updated versions of .sops.yaml and postgres-pwd.enc.yaml.

### Github Actions configuration

Github Action will build the project when pushed to 'master'.

The following Github secrets are needed:

* GCP_CREDENTIALS - GCP Credentials of the GCP GitHub service account
  * Can be shown in terminal with `make gcp-infra-preq gcp-infra-up gcp-show-github-sa-key`
* GKE_PROJECT - Name of the Google cloud project
* SOPS_PRIVATE_KEY - Private key used to encrypt the secrets, see the 'Init secrets'.
* DOCKER_PASSWORD - Dockerhub password, only needed for pingpong and mainapp
* DOCKER_USERNAME - Dockerhub username, only needed for pingpong and mainap

## Deployment

### Deploy manifests

Github action will automatically deploy project manifests to Kubernetes.

## Testing

```bash
$ kubectl get ingress --namespace hy-kube-project
NAME              CLASS    HOSTS   ADDRESS          PORTS   AGE
backend-ingress   <none>   *       34.111.105.205   80      40m

$ curl 34.117.104.209
<!doctype html><html lang="en"><head><meta charset="utf-8"/><link rel="icon" href="/favicon.ico"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content="#000000"/><meta name="description" content="Web site created using create-react-app"/><link rel="apple-touch-icon" href="/logo192.png"/><title>hy-kube app!</title>...

$ curl 34.111.105.205/api/todos
{"todos":[]}%

$ curl -X POST 34.111.105.205/api/todo \
   -H 'Content-Type: application/json' \
   -d '{"task":"Hello GKE!"}'
{"task":"Hello GKE!"}%

$ curl 34.111.105.205/api/todos
{"todos":[{"task":"Hello GKE!"}]}%
```

Tested that adding and getting TODOs works from a browser as well.

## Destroy cluster & other needed GCP resources

```
make gcp-infra-down
```
