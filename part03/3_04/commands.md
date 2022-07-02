# Ex 3.04

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.04
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

### Push to develop, GitHub Actions & test:

Github Actions deployment output:
```
namespace/hy-kube-project-develop created

configmap/backend-config created
configmap/project-db-config created
secret/postgres***
service/backend-svc created
service/frontend-svc created
service/project-db-svc created
persistentvolumeclaim/project-claim created
deployment.apps/backend-dep created
deployment.apps/frontend-dep created
statefulset.apps/postgres-ss created
cronjob.batch/randomtask created
ingress.networking.k8s.io/backend-ingress created

Waiting for deployment "backend-dep" rollout to finish: 1 old replicas are pending termination...
Waiting for deployment "backend-dep" rollout to finish: 1 old replicas are pending termination...
deployment "backend-dep" successfully rolled out

Waiting for deployment "frontend-dep" rollout to finish: 1 old replicas are pending termination...
Waiting for deployment "frontend-dep" rollout to finish: 1 old replicas are pending termination...
deployment "frontend-dep" successfully rolled out
```

```bash
$ kubectl get all --namespace hy-kube-project-develop

NAME                                READY   STATUS    RESTARTS   AGE
pod/backend-dep-5786959f7d-bh2qp    1/1     Running   0          8m45s
pod/frontend-dep-85ddcdc779-d6bsc   1/1     Running   0          8m45s
pod/postgres-ss-0                   1/1     Running   0          24m

NAME                     TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)          AGE
service/backend-svc      NodePort    10.43.250.23   <none>        3456:30674/TCP   24m
service/frontend-svc     NodePort    10.43.254.67   <none>        2345:32045/TCP   24m
service/project-db-svc   ClusterIP   None           <none>        5432/TCP         24m

NAME                           READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/backend-dep    1/1     1            1           24m
deployment.apps/frontend-dep   1/1     1            1           24m

NAME                                      DESIRED   CURRENT   READY   AGE
replicaset.apps/backend-dep-5786959f7d    1         1         1       8m46s
replicaset.apps/backend-dep-644cc4d779    0         0         0       24m
replicaset.apps/frontend-dep-664f9bd64d   0         0         0       24m
replicaset.apps/frontend-dep-85ddcdc779   1         1         1       8m45s

NAME                           READY   AGE
statefulset.apps/postgres-ss   1/1     24m

NAME                       SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/randomtask   0 8 * * *   False     0        <none>          24m
```

### Push to master, GitHub Actions & test:

Github Actions deployment output:
```
namespace/hy-kube-project-master created

configmap/backend-config created
configmap/project-db-config created
secret/postgres***
service/backend-svc created
service/frontend-svc created
service/project-db-svc created
persistentvolumeclaim/project-claim created
deployment.apps/backend-dep created
deployment.apps/frontend-dep created
statefulset.apps/postgres-ss created
cronjob.batch/randomtask created
ingress.networking.k8s.io/backend-ingress created

Waiting for deployment "backend-dep" rollout to finish: 0 of 1 updated replicas are available...
deployment "backend-dep" successfully rolled out

deployment "frontend-dep" successfully rolled out
```

```
$ kubectl get all --namespace hy-kube-project-master

NAME                                READY   STATUS    RESTARTS   AGE
pod/backend-dep-8bfb5799f-5q4gm     1/1     Running   0          2m7s
pod/frontend-dep-8494d44656-gz98j   1/1     Running   0          2m6s
pod/postgres-ss-0                   1/1     Running   0          2m6s

NAME                     TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)          AGE
service/backend-svc      NodePort    10.43.254.53   <none>        3456:32639/TCP   2m10s
service/frontend-svc     NodePort    10.43.246.22   <none>        2345:32233/TCP   2m9s
service/project-db-svc   ClusterIP   None           <none>        5432/TCP         2m9s

NAME                           READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/backend-dep    1/1     1            1           2m8s
deployment.apps/frontend-dep   1/1     1            1           2m7s

NAME                                      DESIRED   CURRENT   READY   AGE
replicaset.apps/backend-dep-8bfb5799f     1         1         1       2m9s
replicaset.apps/frontend-dep-8494d44656   1         1         1       2m8s

NAME                           READY   AGE
statefulset.apps/postgres-ss   1/1     2m8s

NAME                       SCHEDULE    SUSPEND   ACTIVE   LAST SCHEDULE   AGE
cronjob.batch/randomtask   0 8 * * *   False     0        <none>          2m8s
```

## Destroy cluster & other needed GCP resources

```
make gcp-infra-down
```
