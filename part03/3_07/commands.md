# Ex 3.07

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.07
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

Github Action will build the project when pushed to 'master' or 'develop'.

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

The namespace of the deployment will be:
hy-kube-project-{master,develop}

## Testing

```
$ kubectl get ingress --namespace hy-kube-project-master
NAME              CLASS    HOSTS   ADDRESS          PORTS   AGE
backend-ingress   <none>   *       34.111.105.205   80      19m

$ curl 34.111.105.205/api/todos
{"todos":[]}%

$ curl -X POST 34.111.105.205/api/todo \
   -H 'Content-Type: application/json' \
   -d '{"task":"Hello GKE!"}'
{"task":"Hello GKE!"}%

$ curl 34.111.105.205/api/todos
{"todos":[{"task":"Hello GKE!"}]}%
```

And after some visits to the page, there is traffic in Cloud SQL Dashboard:

![image info](./gcloud_sql_postgres_transactions.png)
