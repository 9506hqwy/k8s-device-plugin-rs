# Kubernetes Device Plugin Interface for Rust

## Sample Device

Deploy pod with sample devices.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: demo-dp
  namespace: default
spec:
  containers:
  - image: nginx
    name: nginx
    ports:
    - containerPort: 80
      protocol: TCP
    resources:
      limits:
        demo/sample-device: 2
```
