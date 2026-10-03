# Security Policy

## Reporting a Vulnerability

We take security issues seriously. If you discover a security vulnerability in py2rs, please report it responsibly.

**Please DO NOT open a public issue.** Instead, report security vulnerabilities via:

- Email: [security contact - add your email]
- Security advisory: Use GitHub's "Report a vulnerability" feature (if enabled)

## What to Include

When reporting a vulnerability, please include:

- **Description**: Clear description of the vulnerability
- **Impact**: What an attacker could achieve
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Affected Versions**: Which versions are affected
- **Proposed Fix**: If you have suggestions for fixing the issue

## Response Timeline

- **Initial Response**: Within 48 hours of report
- **Status Update**: Within 7 days with assessment
- **Fix Timeline**: Depends on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: 60 days

## Security Considerations

### API Keys

py2rs stores API keys securely using your system's credential manager:
- **Windows**: Windows Credential Manager
- **macOS**: Keychain
- **Linux**: Secret Service API (libsecret)

Keys are never stored in plain text in configuration files or logs.

### LLM Provider Communication

- All API communications use HTTPS/TLS
- API keys are transmitted securely in request headers
- No sensitive data is logged to disk

### Code Execution

py2rs executes `cargo check` and `cargo build` on generated Rust code:
- Code is executed in the context of the user's project directory
- Standard Rust compiler sandboxing applies
- No arbitrary Python code execution occurs

### Generated Code Review

**Important**: Always review generated Rust code before using in production. LLM-generated code may contain:
- Logic errors
- Security vulnerabilities
- Unsafe memory operations
- Improper error handling

## Known Limitations

- **LLM Prompt Injection**: Malicious Python code comments could potentially influence LLM output
- **Dependency Suggestions**: Suggested Rust crates are not verified for security
- **Network Exposure**: Local model endpoints (Ollama, llama.cpp) may expose localhost ports

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Best Practices

When using py2rs:

1. **Review Generated Code**: Manually inspect all generated Rust code
2. **Verify Dependencies**: Check suggested crates on crates.io for security advisories
3. **Secure API Keys**: Never commit API keys to version control
4. **Use Trusted Models**: Only connect to trusted LLM providers
5. **Network Security**: If using local models, ensure proper firewall configuration
6. **Update Regularly**: Keep py2rs and Rust toolchain updated

## Disclosure Policy

- We follow coordinated vulnerability disclosure
- Security patches will be released as soon as possible
- Public disclosure will occur after fix is available
- Credit will be given to reporters (if desired)

## Security Updates

Security updates will be announced via:
- GitHub Security Advisories
- Release notes with `[SECURITY]` tag
- Project README

---

Thank you for helping keep py2rs secure!
