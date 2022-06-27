provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
}

data "google_container_engine_versions" "default" {
  location       = var.region
  version_prefix = "1.22."
}

resource "google_container_cluster" "primary" {
  name     = "${var.project_id}-gke-cluster"
  location = var.region

  # We can't create a cluster with no node pool defined, but we want to only use
  # separately managed node pools. So we create the smallest possible default
  # node pool and immediately delete it.
  # Ref:
  # https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/container_cluster
  remove_default_node_pool = true
  initial_node_count       = 1
}

resource "google_container_node_pool" "primary_preemptible_nodes" {
  name       = "${var.project_id}-node-pool"
  location   = var.region
  cluster    = google_container_cluster.primary.name
  
  node_count = 1
  version = data.google_container_engine_versions.default.latest_node_version

  node_config {
    preemptible  = true
    machine_type = "e2-medium"
    oauth_scopes    = [
      "https://www.googleapis.com/auth/cloud-platform"
    ]
  }
}
