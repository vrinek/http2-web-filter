# Variables for GitHub Terraform configuration

variable "github_token" {
  description = "GitHub Personal Access Token with repo and admin:repo_hook scopes"
  type        = string
  sensitive   = true
  default     = ""  # Will be read from GITHUB_TOKEN env var
}

variable "repository_name" {
  description = "Name of the GitHub repository"
  type        = string
  default     = "http2-web-filter"
}

variable "default_branch" {
  description = "Default branch name to protect"
  type        = string
  default     = "main"
}

variable "required_approvals" {
  description = "Number of required PR approvals"
  type        = number
  default     = 1
}

variable "dismiss_stale_reviews" {
  description = "Dismiss stale PR approvals when new commits are pushed"
  type        = bool
  default     = true
}

variable "require_up_to_date" {
  description = "Require branches to be up to date before merging"
  type        = bool
  default     = true
}
