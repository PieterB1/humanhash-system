# HumanHash Operational Guide

## Deployment
- Use Helm charts in `helm/humanhash/` for Kubernetes deployment.
- Configure Vault with `security/vault-config.yaml`.

## Maintenance
- Monitor with Prometheus/Grafana.
- Run chaos tests with Chaos Mesh (`kubernetes/chaos-biometric.yaml`).

## Backup
- Backup PostgreSQL database daily.
- Store Vault keys securely.
