# Tools
CD=cd
DOCKER=docker
GCLOUD=gcloud
K=kubectl
TERRAFORM=terraform
SLEEP=sleep
SOPS=sops

# Configuration
GCP_REGION=europe-north1
GCP_PROJECT_ID=dwk-gke-354607
POSTGRES_URL=postgres://postgres:suchSecret@localhost/postgres

# Utilities
PWD = $(shell pwd)

# Global ENV
export TF_VAR_region=$(GCP_REGION)
export TF_VAR_default_zone=$(GCP_REGION)-a
export TF_VAR_project_id=$(GCP_PROJECT_ID)

start-db:
	$(DOCKER) run --name pingpong-postgres -p 5432:5432 -e POSTGRES_PASSWORD=suchSecret -d postgres
	$(SLEEP) 5

stop-db:
	$(DOCKER) stop pingpong-postgres
	$(DOCKER) rm pingpong-postgres

start-prom:
	$(DOCKER) run \
		-p 9090:9090 \
		-v $(PWD)/prometheus_local.yml:/etc/prometheus/prometheus.yml \
		--name prometheus-hy-kube \
		-d \
		prom/prometheus

stop-prom:
	$(DOCKER) stop prometheus-hy-kube
	$(DOCKER) rm prometheus-hy-kube

run-pingpong: stop-db start-db
	$(CD) pingpong && \
	DATABASE_URL=$(POSTGRES_URL) ROCKET_DATABASES='{pingpongdb={url="$(POSTGRES_URL)"}}' cargo run

run-project-backend: stop-db start-db
	$(CD) project/backend && \
	DATABASE_URL=$(POSTGRES_URL) ROCKET_DATABASES='{projectdb={url="$(POSTGRES_URL)"}}' cargo run --bin server

gcp-infra-preq:
	$(CD) gcp_resources && $(TERRAFORM) init

gcp-infra-plan:
	$(CD) gcp_resources && $(TERRAFORM) plan

gcp-infra-up:
	$(CD) gcp_resources && $(TERRAFORM) apply

gcp-infra-down:
	$(CD) gcp_resources && $(TERRAFORM) destroy

gcp-infra-format:
	$(CD) gcp_resources && $(TERRAFORM) fmt

gcp-sync-kubectl-creds:
	$(GCLOUD) container clusters get-credentials $(GCP_PROJECT_ID)-gke-cluster --zone=$(GCP_REGION)-a

gcp-show-github-sa-key:
	@$(CD) gcp_resources && $(TERRAFORM) output -raw github_sa_key | base64 -d

encrypt-project-secrets:
	$(CD) project/backend/manifests/secrets \
	SOPS_AGE_KEY_FILE=$(PWD)/project/backend/manifests/secrets/key.txt \
	&& $(SOPS) --encrypt postgres-pwd.yaml > postgres-pwd.enc.yaml

encrypt-pingpong-secrets:
	$(CD) pingpong/manifests/secrets \
	SOPS_AGE_KEY_FILE=$(PWD)/pingpong/manifests/secrets/key.txt \
	&& $(SOPS) --encrypt postgres-pwd.yaml > postgres-pwd.enc.yaml

apply-pingpong-kube-preq:
	$(K) apply -f manifests_global/mainapp-namespace.yaml
	$(CD) pingpong/manifests/secrets \
	&& SOPS_AGE_KEY_FILE=$(PWD)/pingpong/manifests/secrets/key.txt $(SOPS) --decrypt postgres-pwd.enc.yaml | kubectl apply -f -

apply-pingpong-kube: apply-pingpong-kube-preq
	$(CD) pingpong && $(K) apply -f manifests/

apply-mainapp-kube-preq:
	$(K) apply -f manifests_global/mainapp-namespace.yaml

set-project-postgres-password:
	$(eval POSTGRES_PASS := $(shell kubectl get secrets/postgres-password --namespace hy-kube-project --template {{.data.PASSWORD}} | base64 -d))
	$(eval SQL_INSTANCE := $(shell gcloud sql instances list --format json | jq '.[]|select(.name | startswith("$(GCP_PROJECT_ID)"))' | jq '.name'))
	@gcloud sql users set-password postgres --instance $(SQL_INSTANCE) --password $(POSTGRES_PASS)

apply-project-postgres-ip-secret:
	$(eval IP_ADDR := $(shell gcloud sql instances list --format json | jq '.[]|select(.name | startswith("$(GCP_PROJECT_ID)"))' | jq '.ipAddresses[0].ipAddress'))
	$(K) delete secret postgres-ip --namespace hy-kube-project || true
	$(K) create secret generic --namespace hy-kube-project postgres-ip --from-literal=POSTGRES_IP=$(IP_ADDR)

apply-project-kube-namespace:
	$(K) create namespace hy-kube-project || true

apply-project-kube-preq: apply-project-kube-namespace apply-project-postgres-ip-secret
	:

apply-project-kube-manifests:
	$(CD) project \
	&& SOPS_AGE_KEY_FILE=$(PWD)/project/backend/manifests/secrets/key.txt \
	$(K) kustomize --enable-alpha-plugins . | $(K) apply -f -

apply-project-kube: apply-project-kube-preq apply-project-kube-manifests  set-project-postgres-password
	:

apply-mainapp-kube: apply-mainapp-kube-preq
	$(CD) mainapp && $(K) apply -f manifests/

delete-pingpong-kube:
	$(CD) pingpong && $(K) delete -f manifests/

delete-mainapp-kube:
	$(CD) mainapp && $(K) delete -f manifests/

delete-project-kube:
	$(K) delete namespace hy-kube-project && $(K) create namespace hy-kube-project

loadtest-venv:
	$(CD) load_testing && \
	python3 -m venv .venv && \
	source .venv/bin/activate && \
	python -m pip install -r requirements.txt

loadtest-start-server: loadtest-venv
	$(CD) load_testing && source .venv/bin/activate && python -m locust