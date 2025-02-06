Simple service

Testing :

```bash
grpcurl -plaintext -d '{"a": 3, "b": 4}' localhost:50051 calculator.Calculator/Add
```

or via grpcui:
```bash
grpcui -plaintext localhost:50051
```