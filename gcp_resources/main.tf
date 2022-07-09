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

resource "google_service_account" "github" {
  account_id   = "github-sa"
  display_name = "GitHub Service Account"
}

resource "google_service_account_key" "github_key" {
  service_account_id = google_service_account.github.name
}

resource "google_project_iam_binding" "github_sa_kubernetes_developer_binding" {
  project = var.project_id
  role    = "roles/container.developer"
  members = ["serviceAccount:${google_service_account.github.email}"]
}

resource "google_artifact_registry_repository" "default" {
  provider = google-beta

  location      = var.region
  repository_id = "${var.project_id}-default"
  format        = "DOCKER"
}

resource "google_artifact_registry_repository_iam_member" "github_default_writer" {
  provider = google-beta

  location   = google_artifact_registry_repository.default.location
  repository = google_artifact_registry_repository.default.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${google_service_account.github.email}"
}

resource "google_compute_global_address" "sql_private_ip_address" {
  name          = "private-ip-address"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.gke_vpc.self_link
}

resource "google_service_networking_connection" "sql_private_vpc_connection" {
  network                 = google_compute_network.gke_vpc.self_link
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.sql_private_ip_address.name]
}

resource "random_id" "db_suffix" {
  byte_length = 4
}

resource "google_sql_database_instance" "project_db" {
  depends_on = [
    google_service_networking_connection.sql_private_vpc_connection
  ]

  name             = "${var.project_id}-project-db-${random_id.db_suffix.dec}"
  database_version = "POSTGRES_14"
  region           = var.region

  # This is for development only,
  # no real production use case
  deletion_protection = false

  settings {
    tier = "db-f1-micro"

    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.gke_vpc.id
    }
  }
}

resource "google_sql_database" "project_sql_db" {
  name     = "projectdb"
  instance = google_sql_database_instance.project_db.name
}

resource "google_sql_user" "postgres" {
  name     = "postgres"
  instance = google_sql_database_instance.project_db.name
  password = "suchSecret"
}

resource "google_compute_network" "gke_vpc" {
  name                    = "${var.project_id}-kube-vpc"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "gke_private_subnet" {
  name          = "${var.project_id}-kube-private-subnet"
  region        = var.region
  network       = google_compute_network.gke_vpc.self_link
  ip_cidr_range = "192.168.0.0/16"
}

resource "google_container_cluster" "primary" {
  name     = "${var.project_id}-gke-cluster"
  location = var.region

  networking_mode = "VPC_NATIVE"
  network         = google_compute_network.gke_vpc.self_link
  subnetwork      = google_compute_subnetwork.gke_private_subnet.self_link

  ip_allocation_policy {
    cluster_ipv4_cidr_block  = "/16"
    services_ipv4_cidr_block = "/16"
  }

  # We can't create a cluster with no node pool defined, but we want to only use
  # separately managed node pools. So we create the smallest possible default
  # node pool and immediately delete it.
  # Ref:
  # https://registry.terraform.io/providers/hashicorp/google/latest/docs/resources/container_cluster
  remove_default_node_pool = true
  initial_node_count       = 1
}

resource "google_container_node_pool" "primary_preemptible_nodes" {
  name     = "${var.project_id}-node-pool"
  location = var.region
  cluster  = google_container_cluster.primary.name

  node_count = 1
  version    = data.google_container_engine_versions.default.latest_node_version

  node_config {
    preemptible  = true
    machine_type = "e2-medium"
    oauth_scopes = [
      "https://www.googleapis.com/auth/cloud-platform"
    ]
  }
}

output "github_sa_key" {
  sensitive = true
  value     = google_service_account_key.github_key.private_key
}