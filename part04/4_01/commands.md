# Ex 4.01

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex4.01
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
# pingpong/manifests/secrets/.sops.yaml
creation_rules:
  - encrypted_regex: "^(data)$"
    age: "age1k0upvtn0gwftpep5kxq47xztxj7ulmfhk6t9ha82sd6r5jrjsegsdr0wua" # FIXME
```

Encrypt the key:
```
make encrypt-pingpong-secrets
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


Normally, we would deploy this with:

```
make apply-pingpong-kube apply-mainapp-kube
```

However, as in this case we want to test the behaviour of readiness probes, see the next `Testing` section.

## Testing

### Deploy mainapp

Apply mainapp manifests:

```
make apply-mainapp-kube
```

Check rediness:

```
$ kubectl get po --namespace mainapp
NAME                           READY   STATUS    RESTARTS   AGE
mainapp-dep-7fb98dbb76-zpjfp   1/2     Running   0          17m
```

### Deploy pingpong

Run preqs for pingpong:

```
make apply-pingpong-kube-preq
```

Apply the manifests without postgres-service:

```
kubectl apply -f pingpong/manifests/deployment-config.yaml
kubectl apply -f pingpong/manifests/postgres-config.yaml
kubectl apply -f pingpong/manifests/deployment.yaml
kubectl apply -f pingpong/manifests/service.yaml
kubectl apply -f pingpong/manifests/ingress.yaml
kubectl apply -f pingpong/manifests/postgres-stateful.yaml
```

Check rediness:

```
$ kubectl get po --namespace mainapp
NAME                           READY   STATUS             RESTARTS      AGE
mainapp-dep-7b5f885898-tdbn5   1/2     Running            0             3m53s
pingpong-dep-bbbc97455-z45bc   0/1     CrashLoopBackOff   2 (23s ago)   54s
postgres-ss-0                  1/1     Running            0             53s
```

The way the pingpong application works, is that it creates the connection pool when starting the application. Therefore, the application is crashing at the init state.

Apply postgres service:
```
kubectl apply -f pingpong/manifests/postgres-service.yaml
```

Check rediness:

```
$ kubectl get po --namespace mainapp
NAME                           READY   STATUS        RESTARTS        AGE
mainapp-dep-7b5f885898-tdbn5   2/2     Running       0               8m39s
pingpong-dep-bbbc97455-z45bc   1/1     Running       5 (3m56s ago)   5m40s
postgres-ss-0                  1/1     Running       0               5m39s
```

All works fine now! To test the rediness filter of the pingpong app, we can delete the postgres related stuff again:

```
kubectl delete -f pingpong/manifests/postgres-service.yaml
kubectl delete -f pingpong/manifests/postgres-stateful.yaml
```

Check rediness:

```
$ kubectl get po --namespace mainapp
NAME                           READY   STATUS    RESTARTS      AGE
mainapp-dep-7b5f885898-tdbn5   1/2     Running   0             14m
pingpong-dep-bbbc97455-z45bc   0/1     Running   5 (10m ago)   11m
```

Add postgres again:
```
kubectl apply -f pingpong/manifests/postgres-service.yaml
kubectl apply -f pingpong/manifests/postgres-stateful.yaml
```

Check rediness:

```
$ kubectl get po --namespace mainapp
NAME                           READY   STATUS    RESTARTS      AGE
mainapp-dep-7b5f885898-tdbn5   2/2     Running   0             16m
pingpong-dep-bbbc97455-z45bc   1/1     Running   5 (12m ago)   14m
postgres-ss-0                  1/1     Running   0             72s
```