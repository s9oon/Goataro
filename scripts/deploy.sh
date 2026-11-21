#!/usr/bin/env bash
# Deploy Goataro to whichever cluster kubectl is currently pointed at.
#
# Usage (from the repo root):
#   bash scripts/deploy.sh <commit-sha>
#
# Needs k8s/secrets/postgres.env and k8s/secrets/backend.env (not in git).
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "usage: bash scripts/deploy.sh <commit-sha>" >&2
  exit 1
fi

SHA="$1"

echo "Deploying to cluster: $(kubectl config current-context)"

# Create the Secrets, or update them if they already exist.
kubectl create secret generic postgres-secrets \
  --from-env-file=k8s/secrets/postgres.env \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl create secret generic backend-secrets \
  --from-env-file=k8s/secrets/backend.env \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/service.yaml

# Swap the IMAGE_TAG placeholder for the real tag without editing the file.
sed "s|IMAGE_TAG|$SHA|" k8s/deployment.yaml | kubectl apply -f -
