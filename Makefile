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
export TF_VAR_project_id=$(GCP_PROJECT_ID)

start-db:
	$(DOCKER) run --name pingpong-postgres -p 5432:5432 -e POSTGRES_PASSWORD=suchSecret -d postgres
	$(SLEEP) 5

stop-db:
	$(DOCKER) stop pingpong-postgres
	$(DOCKER) rm pingpong-postgres

run-pingpong: stop-db start-db
	$(CD) pingpong && \
	DATABASE_URL=$(POSTGRES_URL) ROCKET_DATABASES='{pingpongdb={url="$(POSTGRES_URL)"}}' cargo run

gcp-infra-preq:
	$(CD) gcp_resources && $(TERRAFORM) init

gcp-infra-plan:
	$(CD) gcp_resources && $(TERRAFORM) plan

gcp-infra-up:
	$(CD) gcp_resources && $(TERRAFORM) apply

gcp-infra-down:
	$(CD) gcp_resources && $(TERRAFORM) destroy

gcp-sync-kubectl-creds:
	gcloud container clusters get-credentials $(GCP_PROJECT_ID)-gke-cluster --zone=$(GCP_REGION)

apply-pingpong-kube-preq:
	$(K) apply -f manifests_global/mainapp-namespace.yaml
	$(CD) pingpong/manifests/secrets \
	&& SOPS_AGE_KEY_FILE=$(PWD)/pingpong/manifests/secrets/key.txt $(SOPS) --decrypt postgres-pwd.enc.yaml | kubectl apply -f -

apply-pingpong-kube: apply-pingpong-kube-preq
	$(CD) pingpong && $(K) apply -f manifests/

apply-mainapp-kube-preq:
	$(K) apply -f manifests_global/mainapp-namespace.yaml

apply-mainapp-kube: apply-mainapp-kube-preq
	$(CD) mainapp && $(K) apply -f manifests/

delete-pingpong-kube:
	$(CD) pingpong && $(K) delete -f manifests/

delete-mainapp-kube:
	$(CD) mainapp && $(K) delete -f manifests/
