# Security Policy

At Dodo, security is a top priority. We’re committed to maintaining a secure codebase and ensuring that our CI/CD automation workflows are robust against potential threats. This policy outlines how we handle security concerns, review contributions, and respond to vulnerability reports.

## Reporting Vulnerabilities

If you discover a security vulnerability in Dodo (or related components), please report it responsibly by emailing us at **tanvipm19@gmail.com**. When reporting, please include:
- A clear description of the vulnerability
- Steps to reproduce the issue
- Any relevant logs or screenshots
- Your contact information (so we can follow up if needed)

We will acknowledge your report within 48 hours and work diligently to address the issue.

## Security Guidelines for Contributors

To help us maintain a secure project, please adhere to the following guidelines when contributing:

- **Do Not Hardcode Secrets:**  
  Avoid including any sensitive information, such as API keys, passwords, or tokens, directly in the code or configuration files.

- **Validate Inputs:**  
  Ensure that any user-provided input is properly validated and sanitized to prevent injection attacks or data leakage.

- **Follow the Principle of Least Privilege:**  
  Write code that operates with the minimum permissions necessary to perform its task. This applies to both your application code and CI/CD workflows.

- **Regularly Update Dependencies:**  
  Keep all dependencies up-to-date. Use automated tools (e.g., Dependabot) to monitor and update packages to their latest secure versions.

- **Secure Configuration Parsing:**  
  When parsing configuration files, use safe parsing libraries and handle errors gracefully to avoid exposing sensitive information.

- **Implement Robust Error Handling:**  
  Ensure that your error handling does not leak sensitive data or system details.

- **Use Automated Security Scanning:**  
  Incorporate static analysis and vulnerability scanning tools into your development and CI/CD pipelines.

## Our Security Practices

- **Code Reviews:**  
  Every contribution undergoes a thorough code review, which includes security audits of any changes affecting workflow or configuration files.

- **Automated Scanning:**  
  We integrate automated static analysis and security scanning tools into our CI/CD pipelines to catch vulnerabilities early.

- **Immutable Workflow Templates:**  
  Once published, workflow templates are versioned and immutable. Any updates require a new version, minimizing the risk of unauthorized changes.

- **Continuous Monitoring:**  
  We regularly monitor our systems for suspicious activities and continuously update our security practices to align with industry standards.

## Incident Response

- **Acknowledgment:**  
  We will acknowledge all security reports within 48 hours of receipt.
- **Investigation:**  
  Reported vulnerabilities are prioritized and investigated promptly.
- **Communication:**  
  We will communicate with the reporter about the status of the issue and, if applicable, publicly disclose the fix once the vulnerability is resolved.
- **Post-Incident Review:**  
  After resolving any incident, we will conduct a post-incident review to improve our processes and prevent future occurrences.

## Continuous Improvement

Security is an ongoing commitment. We welcome feedback and contributions to our security practices. Please review our [Code of Conduct](CODE_OF_CONDUCT.md) and [LICENSE](LICENSE) files to understand our expectations for a secure and respectful environment.

Thank you for helping us maintain a secure project and for contributing to the safety and reliability of Dodo.
