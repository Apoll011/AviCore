# Security Policy

## Supported Versions

AVI Core follows a conservative support model. Only explicitly listed versions receive security updates.
Security fixes are applied to the latest stable release first. Backports are performed only when the risk justifies the operational cost.

## Reporting a Vulnerability

Security issues **must not** be reported via public issues, pull requests, or discussions.

To report a vulnerability:

* Open a **GitHub Security Advisory** for this repository, or
* Contact the maintainers privately via the security contact listed in the repository metadata.

A good report includes:

* A clear description of the issue
* Steps to reproduce or a proof of concept
* Impact assessment (what can be read, executed, or disrupted)
* Affected versions or commits, if known

## Response Expectations

* **Acknowledgement:** within 72 hours
* **Initial assessment:** within 7 days
* **Fix or mitigation:** timeline depends on severity and complexity

Critical vulnerabilities are prioritized and may result in an immediate patch release.

## Disclosure Policy

* Responsible disclosure is expected.
* Please allow reasonable time for a fix before any public disclosure.
* Credit will be given where appropriate, unless anonymity is requested.

## Scope

In scope:

* AVI Core runtime
* Skill loading and execution model
* Namespace, module, and permission systems
* Communication and protocol handling

Out of scope:

* Vulnerabilities requiring a fully compromised host OS
* Issues in third-party dependencies without an AVI-specific exploit path

## Final Note

Security in AVI Core is intentional, capability-driven, and fail-closed by design. Reports that improve correctness, isolation, or predictability are always welcome.
