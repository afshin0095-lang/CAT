# A05 — Agent Registry

The registry is the authoritative catalog of deployable agent definitions.

## Registration record

`agent_id`, version, class, description, input schema, output schema, capability IDs, policy profile, budget profile, model/provider adapters, enabled state, owner, compatibility, and deprecation date.

Registry changes are versioned and auditable. Disabled or deprecated agents cannot receive new privileged work. Runtime configuration may select an implementation, but cannot expand the registered capability set without policy approval.
