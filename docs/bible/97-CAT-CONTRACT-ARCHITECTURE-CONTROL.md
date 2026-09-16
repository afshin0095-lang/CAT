# CAT Contract Architecture Control Note

The contract layer is governed by one rule above all others:

> Stable semantic contracts must remain independent from volatile implementation providers.

Every future Agent, Capability, Tool, Connector and Provider should preserve this boundary unless an explicit architectural decision changes it.

The next implementation phase begins with the current codebase and tests.
