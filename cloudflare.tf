variable takehome_cloudflare_email {
  type = string
}

variable takehome_cloudflare_api_token {
  type = string
}

variable takehome_cloudflare_account_id {
  type = string
}

variable takehome_cloudflare_zone {
  type = string
}

variable takehome_cloudflare_zone_id {
  type = string
}

variable takehome_cloudflare_domain {
  type = string
}

provider cloudflare {
  version = "~> 2.0"
  account_id   = var.takehome_cloudflare_account_id
  email   = var.takehome_cloudflare_email
  api_token = var.takehome_cloudflare_api_token
}

resource cloudflare_record root {
  zone_id = var.takehome_cloudflare_zone_id
  name    = var.takehome_cloudflare_domain
  value   = "4.4.4.4" // Doesn't matter, since we'll use workers
  type    = "A"
  ttl     = 1
  proxied = true
}

resource cloudflare_worker_script blog {
  name = "blog"
  content = file("dist/index.js")
}

# Runs the specified worker script for all URLs that match `example.com/*`
resource cloudflare_worker_route blog {
  zone_id = var.takehome_cloudflare_zone_id
  pattern = "*${var.takehome_cloudflare_zone}/*"
  script_name = cloudflare_worker_script.blog.name
}
