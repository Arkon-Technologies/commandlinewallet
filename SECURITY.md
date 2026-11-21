# Security policy

CLW Phase 1 has **no operational wallet security features**. It must not be used to hold funds or process real transaction data. Its current security documents describe intended boundaries and limitations, not guarantees.

To report a vulnerability, contact the repository maintainers privately through the repository's GitHub security advisory channel when available. Do not post secrets, seed phrases, or exploit details in public issues. The maintainers have not declared a response-time SLA.

Security-critical changes should be reviewed against [the threat model](docs/THREAT_MODEL.md) and [the security model](docs/SECURITY_MODEL.md). In particular, new code must never log secrets, expose private keys to the terminal or network coordinator, or allow unrestricted arbitrary-message signing.
