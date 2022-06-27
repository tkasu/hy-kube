# Ex 3.02

## Checking out the correct version

To rebuild the same image, first checkout the correct tag:

```
git checkout ex3.02
```

## Building the applications

Push the tag the github with 'ex'-prefix, e.g. 'ex3.01' and the Github Action will publish the project to Docker Hub repository: 'tkasu/hy-kube-*'

## Create cluster, other needed GCP resources and sync kubectl creds

```
make gcp-infra-preq gcp-infra-up gcp-sync-kubectl-creds
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
make apply-pingpong-kube apply-mainapp-kube
```

## Testing

```bash
$ kubectl get ingress --namespace mainapp
NAME               CLASS    HOSTS   ADDRESS          PORTS   AGE
pingpong-ingress   <none>   *       34.111.105.205   80      6m50s

$ curl 34.111.105.205
Hello
2022-06-27T17:52:16.039670553Z 7159ae2f-a20a-4c8b-8a2d-c3d60cd3a0f1
Ping / Pongs: 0%

$ curl 34.111.105.205/pingpong
pong 0%

$ curl 34.111.105.205/pingpong
pong 1%

$ curl 34.111.105.205
Hello
2022-06-27T17:52:16.039670553Z 7159ae2f-a20a-4c8b-8a2d-c3d60cd3a0f1
Ping / Pongs: 2%
```

## Destroy cluster & other needed GCP resources

```
make gcp-infra-down
```