# Outputs for GitHub Terraform configuration

output "repository_name" {
  description = "Name of the configured repository"
  value       = local.repository_name
}

output "default_branch" {
  description = "Default branch with protection"
  value       = local.default_branch
}

output "branch_protection_id" {
  description = "ID of the branch protection rule"
  value       = github_branch_protection.main.id
}

output "required_checks" {
  description = "List of required status checks"
  value       = github_branch_protection.main.required_status_checks[0].contexts
}
