# GitHub Repository Settings
# This file configures branch protection rules and other repository settings

locals {
  repository_name = "http2-web-filter"
  default_branch  = "main"
}

# Branch protection for main branch
resource "github_branch_protection" "main" {
  repository_id = local.repository_name
  pattern       = local.default_branch

  # Require pull request reviews before merging
  required_pull_request_reviews {
    dismiss_stale_reviews           = true
    require_code_owner_reviews      = false
    required_approving_review_count = 1  # Require 1 approval
  }

  # Require status checks to pass before merging
  required_status_checks {
    strict   = true  # Require branches to be up to date before merging
    contexts = [
      "Build and Test"  # This matches the job name in ci.yml
    ]
  }

  # Do not allow bypassing the above settings
  enforce_admins = false

  # Allow force pushes (set to false to block)
  allows_force_pushes = false

  # Allow deletions
  allows_deletions = false
}

# Repository ruleset (alternative to branch protection, newer GitHub feature)
# Uncomment to use rulesets instead of branch protection
# resource "github_repository_ruleset" "main" {
#   repository  = local.repository_name
#   name        = "main-branch-protection"
#   target      = "branch"
#   enforcement = "active"
#
#   conditions {
#     ref_name {
#       include = ["~DEFAULT_BRANCH"]
#       exclude = []
#     }
#   }
#
#   rules {
#     pull_request {
#       dismiss_stale_reviews_on_push     = true
#       require_code_owner_review         = false
#       require_last_push_approval          = false
#       required_approving_review_count     = 1
#       required_review_thread_resolution   = true
#     }
#
#     required_status_checks {
#       required_check {
#         context = "Build and Test"
#         integration_id = 0
#       }
#       strict_required_status_check_policy = true
#     }
#   }
# }
