# Security Policy

## Supported Versions

We release security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.2.x   | :white_check_mark: |
| 1.1.x   | :white_check_mark: |
| 1.0.x   | :x:                |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in tmpltool, please report it by emailing the maintainers. You can find the contact information in the repository.

### What to Include

Please include the following information in your report:

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Potential impact** of the vulnerability
- **Suggested fix** (if you have one)
- **Your contact information** for follow-up

### Response Timeline

- **Initial Response:** We aim to respond to security reports within 48 hours
- **Confirmation:** We will confirm the vulnerability within 5 business days
- **Fix Timeline:** We will work on a fix and aim to release a patch within 14 days for critical vulnerabilities
- **Disclosure:** We will coordinate with you on the disclosure timeline

## Security Best Practices for tmpltool

When using tmpltool, please follow these security best practices:

### 1. Trust Mode (`--trust`)

**⚠️ WARNING:** Only use `--trust` mode with templates you completely trust.

```bash
# ✗ DANGEROUS - Don't use --trust with untrusted templates
tmpltool --trust untrusted_template.tmpl

# ✓ SAFE - Use without --trust for untrusted templates
tmpltool untrusted_template.tmpl
```

**Why?** Trust mode disables security restrictions:
- Can read any file on the system (e.g., `/etc/passwd`, SSH keys)
- Can access parent directories (`../`)
- Can read sensitive configuration files

**Only use `--trust` when:**
- You wrote the template yourself
- You've reviewed and audited the template
- You trust the template source completely
- You need to access system files intentionally

### 2. Template Security

**Avoid processing untrusted templates without review:**

```bash
# ✗ DANGEROUS - Don't process templates from untrusted sources
curl https://untrusted-site.com/template.tmpl | tmpltool

# ✓ SAFE - Review templates before using them
curl https://trusted-site.com/template.tmpl -o template.tmpl
# Review the template
cat template.tmpl
# Then use it
tmpltool template.tmpl
```

**What to check in templates:**
- File system access attempts
- Unexpected environment variable usage
- Suspicious patterns or obfuscation

### 3. Environment Variables

**Be careful with sensitive environment variables:**

```bash
# ✗ DANGEROUS - Don't expose secrets in environment
export DATABASE_PASSWORD="secret123"
tmpltool template.tmpl  # Template could leak this

# ✓ BETTER - Use secure secret management
# Load secrets only when needed and clear them after use
```

**Best practices:**
- Don't put secrets in environment variables if possible
- Use dedicated secret management tools
- Clear sensitive env vars after use
- Review templates for `filter_env(pattern="*")` which could expose all env vars

### 4. Filesystem Access

**Default security (without `--trust`):**
- ✓ Only relative paths allowed
- ✓ No access to parent directories (`..`)
- ✓ No absolute paths (`/etc/passwd`)
- ✓ Restricted to current working directory

**With `--trust` mode:**
- ⚠️ Full filesystem access
- ⚠️ Can read any file the user can access
- ⚠️ No restrictions

### 5. Docker Security

When using Docker:

```bash
# ✓ SAFE - Mount only necessary directories
docker run --rm -v $(pwd):/workspace -w /workspace tmpltool template.tmpl

# ✗ DANGEROUS - Don't mount entire filesystem
docker run --rm -v /:/host tmpltool template.tmpl
```

## Known Security Considerations

### 1. Template Injection

tmpltool uses the Tera template engine. While Tera is designed to be safe, be aware that:
- Templates have access to environment variables via `get_env()`
- Templates can read files via filesystem functions
- Trust mode disables all security restrictions

### 2. Filesystem Access

Filesystem functions enforce security by default:
- `read_file()` - Restricted to relative paths
- `file_exists()` - Restricted to relative paths
- `list_dir()` - Restricted to relative paths
- `glob()` - Restricted to current directory

Use `--trust` only when necessary and with caution.

### 3. Environment Variable Exposure

Functions like `filter_env(pattern="*")` can expose all environment variables. Review templates carefully to ensure they don't leak sensitive information.

## Security Updates

Security updates will be:
1. Developed privately
2. Tested thoroughly
3. Released as patch versions (e.g., 1.2.1)
4. Announced in release notes with `[SECURITY]` tag
5. Added to this SECURITY.md file

## Security Hall of Fame

We'd like to thank the following people for responsibly disclosing security issues:

<!-- Names will be added here as vulnerabilities are reported and fixed -->

---

**Remember:** Security is everyone's responsibility. If you see something, say something!
