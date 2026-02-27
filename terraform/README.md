# GitHub Repository Infrastructure

## Overview

This Terraform configuration manages GitHub repository settings for the http2-web-filter project, including branch protection rules that enforce CI checks.

## Prerequisites

1. **Terraform installed** (>= 1.0)
2. **GitHub Personal Access Token** with these scopes:
   - `repo` - Full control of private repositories
   - `admin:repo_hook` - Full control of repository hooks

## Setup

### 1. Create GitHub Token

1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select scopes: `repo`, `admin:repo_hook`
4. Generate and copy the token

### 2. Set Environment Variable

```bash
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxx"
```

### 3. Initialize Terraform

```bash
cd terraform
terraform init
```

### 4. Plan Changes

```bash
terraform plan
```

### 5. Apply Changes

```bash
terraform apply
```

## What This Configures

### Branch Protection for `main`

- ✅ **Requires pull request** before merging
- ✅ **Requires 1 approval** on pull requests
- ✅ **Dismisses stale reviews** when new commits are pushed
- ✅ **Requires status checks** to pass (CI / Build and Test)
- ✅ **Requires up-to-date branches** before merging
- ❌ **Blocks force pushes**
- ❌ **Blocks branch deletion**

## Customization

Edit `variables.tf` or use command-line flags:

```bash
terraform apply -var="required_approvals=2" -var="dismiss_stale_reviews=false"
```

## State Management

For team use, configure remote state storage in `main.tf`:

```hcl
terraform {
  backend "s3" {
    bucket = "my-terraform-state"
    key    = "http2-web-filter/github.tfstate"
    region = "us-east-1"
  }
}
```

## Destroy

To remove all managed resources:

```bash
terraform destroy
```

⚠️ **Warning**: This will remove branch protection rules!

## Troubleshooting

### Token not working

Ensure your token has the correct scopes. The token owner must have admin access to the repository.

### CI check not found

The status check name must match exactly. Check your `.github/workflows/ci.yml` job name. It defaults to "Build and Test".

### Import existing resources

If rules already exist, import them:

```bash
terraform import github_branch_protection.main http2-web-filter:main
```
