# A01 — Agent Intelligence Principles

1. **Domain authority:** agents never become the source of financial, identity, tracking, or lifecycle truth.
2. **Least authority:** an agent receives only the capabilities required for its current task.
3. **Bounded autonomy:** every run has time, cost, tool, recursion, and output limits.
4. **Deterministic infrastructure:** retries, deduplication, state transitions, and persistence are deterministic even when model output is probabilistic.
5. **Typed boundaries:** model text is untrusted until parsed and validated into a domain contract.
6. **Observable decisions:** important actions carry reason, policy, correlation, and trace metadata.
7. **Provider independence:** model/provider replacement must not require rewriting domain logic.
8. **Fail closed:** missing policy, identity, validation, or budget information blocks privileged actions.
9. **No hidden side effects:** planning is separate from execution and execution is separate from commitment.
10. **Human override:** high-impact or policy-defined operations can require explicit human approval.
