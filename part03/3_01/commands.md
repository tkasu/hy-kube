# Ex 3.01

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.01
```

## Building the applications

Push the tag the github with 'ex'-prefix, e.g. 'ex3.01' and the Github Action will publish the project to Docker Hub repository: 'tkasu/hy-kube-*'

## Create cluster, other needed GCP resources and sync kubectl creds

```
make gcp-infra-preq gcp-infra-up gcp-sync-kubectl-creds
```

## Creating the namespace

```
kubectl apply -f manifests_global/mainapp-namespace.yaml

namespace/mainapp created
```

## Deployment

## Init secrets

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

## Deploy manifests

```
make apply-pingpong-kube
```

## Testing

```bash
$ kubectl get svc --namespace mainapp
NAME           TYPE           CLUSTER-IP      EXTERNAL-IP    PORT(S)        AGE
pingpong-db    ClusterIP      None            <none>         5432/TCP       5m23s
pingpong-svc   LoadBalancer   10.43.247.133   34.88.30.255   80:30070/TCP   5m23s

$ curl 34.88.30.255
pong 0%
```

## Destroy cluster & other needed GCP resources

```
make gcp-infra-down
```