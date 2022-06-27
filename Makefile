# Tools
CD=cd
GCLOUD=gcloud
K=kubectl
TERRAFORM=terraform
SOPS=sops

# Configuration
GCP_REGION=europe-north1
GCP_PROJECT_ID=dwk-gke-354607

# Utilities
PWD = $(shell pwd)

# Global ENV
export TF_VAR_region=$(GCP_REGION)
export TF_VAR_project_id=$(GCP_PROJECT_ID)

gcp-infra-preq:
	$(CD) gcp_resources && $(TERRAFORM) init

gcp-infra-plan:
	$(CD) gcp_resources && $(TERRAFORM) plan

gcp-infra-up:
	$(CD) gcp_resources && $(TERRAFORM) apply

gcp-infra-down:
	$(CD) gcp_resources && $(TERRAFORM) destroy

gcp-sync-kubectl-creds:
	gcloud container clusters get-credentials $(GCP_PROJECT_ID)-gke-cluster --zone=europe-$(GCP_REGION)

apply-pingpong-kube-preq:
	$(K) apply -f manifests_global/mainapp-namespace.yaml
	$(CD) pingpong/manifests/secrets \
	&& SOPS_AGE_KEY_FILE=$(PWD)/pingpong/manifests/secrets/key.txt $(SOPS) --decrypt postgres-pwd.enc.yaml | kubectl apply -f -

apply-pingpong-kube: apply-pingpong-kube-preq
	$(CD) pingpong && $(K) apply -f manifests/
