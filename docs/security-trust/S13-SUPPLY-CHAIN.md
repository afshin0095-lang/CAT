# S13 — Supply Chain Security

CAT protects the software and model supply chain.

Controls include:

- dependency lockfiles and review;
- vulnerability and license scanning;
- pinned CI actions where practical;
- container/image provenance;
- minimal CI permissions;
- protected release artifacts;
- review of third-party SDKs and provider adapters;
- model/provider version tracking;
- no dynamic execution of unverified downloaded code.

A dependency upgrade is a security and compatibility change, not merely a package-manager operation.
