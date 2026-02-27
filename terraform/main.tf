terraform {
  required_version = ">= 1.0"

  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }

  # Uncomment and configure for remote state (recommended for team use)
  # backend "s3" {
  #   bucket = "my-terraform-state-bucket"
  #   key    = "http2-web-filter/github.tfstate"
  #   region = "us-east-1"
  # }
}

provider "github" {
  # Token is read from GITHUB_TOKEN environment variable
  # Ensure your token has 'repo' and 'admin:repo_hook' scopes
}
