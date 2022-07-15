# Ex 4.02

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex4.02
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

After Github Actions have ran, lets check the status of the pods:

```
$ kubectl get po --namespace hy-kube-project-master
NAME                                     READY   STATUS    RESTARTS   AGE
backend-dep-6c469c4956-52pqm             1/1     Running   0          50s
backend-imagesync-dep-7cfd56db9b-28jcj   1/1     Running   0          50s
frontend-dep-74fdc8979f-x8759            1/1     Running   0          49s
```

Lets test backend probes by stopping the SQL instance from the console:

```
$ gcloud sql instances describe dwk-gke-354607-project-db-1271990469 | tail -n1
state: STOPPED
```

Check pods state:

```
$ kubectl get po --namespace hy-kube-project-master
NAME                                     READY   STATUS    RESTARTS      AGE
backend-dep-6c469c4956-52pqm             0/1     Error     3 (22s ago)   11m
backend-imagesync-dep-7cfd56db9b-28jcj   1/1     Running   0             11m
frontend-dep-74fdc8979f-x8759            1/1     Running   0             10m
```

Pod was restarted after healthcheck failed, and failed to start as the server can't init the SQL pool connections (so in this case restart is not really helping).

Start SQL instance again from console and check pods:

```
$ kubectl get po --namespace hy-kube-project-master
NAME                                     READY   STATUS    RESTARTS        AGE
backend-dep-6c469c4956-52pqm             1/1     Running   8 (5m33s ago)   22m
backend-imagesync-dep-7cfd56db9b-28jcj   1/1     Running   0               22m
frontend-dep-74fdc8979f-x8759            1/1     Running   0               22m
```
