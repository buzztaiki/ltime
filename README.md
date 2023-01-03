# ltime -- Local time filter

`ltime` is a command line tool that translates any timezone time to local time.

## Usage examples

```console
% echo '2021-01-02T03:04:05z' | ltime
2021-01-02T12:04:05+09:00

% echo '2021-01-02 03:04:05 -0800' | ltime
2021-01-02T20:04:05+09:00

% kubectl -n kube-system logs kube-apiserver-minikube --timestamps --tail 3 | ltime
2023-01-03T12:59:48.644744691+09:00 I0103 03:59:48.644628       1 controller.go:611] quota admission added evaluator for: events.events.k8s.io
2023-01-03T12:59:57.985596110+09:00 I0103 03:59:57.985502       1 controller.go:611] quota admission added evaluator for: endpoints
2023-01-03T12:59:58.259088694+09:00 I0103 03:59:58.258769       1 controller.go:611] quota admission added evaluator for: endpointslices.discovery.k8s.io
```
