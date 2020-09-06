# Ex 1.10

## Notes

```bash
k # is alias for kubectl 

192.168.99.100 # is my cluster ip, as I'm using virtualbox docker-machine backend (AMD Hackintosh probelms :sadface)
```

## Building mainapp application

To rebuild the same image, first checkout the correct tag:

```
git checkout ex1.10
```

In folder mainapp/

```
./docker_build_and_publish.sh ex1.10
```

## Create cluster

```
k3d cluster create --port '8082:30080@agent[0]' -p 8081:80@loadbalancer --agents 2
```

## Deployment

In folder maniapp/:

Deploy app, service and ingress

```
§ k3d cluster start

§ k apply -f manifests/

deployment.apps/hy-kube-mainapp-dep configured
ingress.extensions/hy-kube-ingress configured
service/hy-kube-mainapp-svc configured
```

Test

```bash
$ k get pods

NAME                                   READY   STATUS    RESTARTS   AGE
hy-kube-mainapp-dep-844756bf5c-9hrsb   2/2     Running   0          7m38s

# Note the warning that app could not read timestamps.txt at the beginning, probably it polled the file before writer wrote the first line
$ k logs hy-kube-mainapp-dep-844756bf5c-9hrsb hy-kube-mainapp-reader | head -15

WARNING: Couldn't open /usr/app/files/timestamps.txt: No such file or directory (os error 2)
Configured for production.
    => address: 0.0.0.0
    => port: 3000
    => log: critical
    => workers: 8
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
Rocket has launched from http://0.0.0.0:3000
2020-09-06T10:35:03.716348317Z: e8c948ae-ac3e-4987-a977-88bb7498d192
2020-09-06T10:35:08.716516104Z: e8c948ae-ac3e-4987-a977-88bb7498d192
2020-09-06T10:35:13.716599077Z: e8c948ae-ac3e-4987-a977-88bb7498d192

```

```bash
$ curl 192.168.99.100:8081     
2020-09-06T10:45:33.758385674Z e8c948ae-ac3e-4987-a977-88bb7498d192% 

$ curl 192.168.99.100:8081
2020-09-06T10:45:38.759077352Z e8c948ae-ac3e-4987-a977-88bb7498d192%

$ curl 192.168.99.100:8081
2020-09-06T10:45:43.759429888Z e8c948ae-ac3e-4987-a977-88bb7498d192%
```

