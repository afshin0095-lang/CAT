# S06 — Secret Management

Secrets are infrastructure credentials, not application data.

## Rules

- Never commit secrets.
- Never place provider keys in frontend bundles.
- Never put secrets in agent prompts or ordinary model context.
- Prefer short-lived credentials and scoped tokens.
- Centralize retrieval through a secret-management boundary.
- Log secret metadata only; never log secret values.
- Rotate and revoke credentials according to provider and risk policy.

Applications receive only the specific secret needed for a narrowly defined operation. Providers must be isolated so a credential for one integration cannot automatically access another.
