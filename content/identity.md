+++
name = "Kennedy Mosoti"
headline = "Platform, Reliability & Systems Automation Engineer"
positioning = "Platform and reliability engineer focused on infrastructure automation, operational debugging, and verifiable system state."
bio = "Kennedy Mosoti is a platform and reliability engineer working across Python, Linux/RHEL, SaltStack, Splunk, Kafka, Logstash, and infrastructure automation. His strongest production work involves debugging long-running automation, improving configuration convergence, constraining privileged changes, and building telemetry that distinguishes request success from actual system state. Independently, he develops Rust and Python systems projects around observability, deterministic control, and reliability for AI-agent runtimes."
location = "Dallas–Fort Worth, Texas"

links = [
  { label = "kennedy.mosoti.dev", href = "https://kennedy.mosoti.dev", kind = "website" },
  { label = "GitHub", href = "https://github.com/kmosoti", kind = "repository" },
  { label = "Email", href = "mailto:kennedy.rmosoti@gmail.com", kind = "email" },
]

# Order is the Core register bank's slot order (06: DisciplineId).
[[disciplines]]
id = "systems"
title = "Systems"
summary = "Production debugging of long-running automation down to its mechanism, using minimal reproduction as evidence; explicit lifecycle state machines and interruption safety are development targets, not established strengths."
bays = ["lab"]

[[disciplines]]
id = "platform"
title = "Platform automation"
summary = "Production fleet lifecycle automation on a large RHEL estate: SaltStack desired state, rebuild and patch-posture automation, least-privilege remediation and staged rollout."
bays = ["infrastructure"]

[[disciplines]]
id = "observability"
title = "Observability"
summary = "Production troubleshooting and configuration along Kafka, Logstash and Splunk ingestion paths, Splunk platform automation, and telemetry that distinguishes a request's success from the system's actual state."
bays = ["observability"]

[[disciplines]]
id = "agentic"
title = "Agent runtimes"
summary = "Independent work, not production experience, on bounded runtimes in which stochastic workers propose and a deterministic runtime owns authority, verification, state and commit."
bays = ["lattice"]

# The six detents of Core's drum, in order.
[[operating_pattern]]
stage = "observe reality"
line = "Read the system's state from an independent signal, not from the request that was meant to change it."

[[operating_pattern]]
stage = "model state"
line = "Name the states the system can be in, and the transitions between them, before acting on it."

[[operating_pattern]]
stage = "constrain mutation"
line = "Limit what a change may touch: the least privilege it needs, a bounded target, a staged gate."

[[operating_pattern]]
stage = "execute"
line = "Apply the change through the platform's own primitives where they exist."

[[operating_pattern]]
stage = "verify convergence"
line = "Confirm by observation that the system reached the desired state, not only that the request was accepted."

[[operating_pattern]]
stage = "preserve recovery"
line = "Keep a way back: retry, resume or rollback still work after interruption or partial success."
+++

## Positioning

A software and infrastructure engineer whose strongest demonstrated work sits at the boundary of platform automation, Linux systems, configuration management, operational reliability and observability. The production evidence is strongest where a system is changing or failing and the visible symptom does not identify the mechanism: long-running automation that appears hung after the underlying workflow completed, fleet configuration that fails to converge after a rebuild, or operational state that cannot be trusted without independent verification. Independent work covers Rust and Python systems projects around observability, deterministic control and reliability for AI-agent runtimes.

## Disciplines

- **Systems.** Production debugging of long-running automation down to its mechanism, using minimal reproduction as evidence; explicit lifecycle state machines and interruption safety are development targets, not established strengths.
- **Platform automation.** Production fleet lifecycle automation on a large RHEL estate: SaltStack desired state, rebuild and patch-posture automation, least-privilege remediation and staged rollout.
- **Observability.** Production troubleshooting and configuration along Kafka, Logstash and Splunk ingestion paths, Splunk platform automation, and telemetry that distinguishes a request's success from the system's actual state.
- **Agent runtimes.** Independent work, not production experience, on bounded runtimes in which stochastic workers propose and a deterministic runtime owns authority, verification, state and commit.

## Operating pattern

Observe reality → model state → constrain mutation → execute → verify convergence → preserve recovery.

- **Observe reality.** Read the system's state from an independent signal, not from the request that was meant to change it.
- **Model state.** Name the states the system can be in, and the transitions between them, before acting on it.
- **Constrain mutation.** Limit what a change may touch: the least privilege it needs, a bounded target, a staged gate.
- **Execute.** Apply the change through the platform's own primitives where they exist.
- **Verify convergence.** Confirm by observation that the system reached the desired state, not only that the request was accepted.
- **Preserve recovery.** Keep a way back: retry, resume or rollback still work after interruption or partial success.

This is a design direction and an operating discipline. It is not a claim that every production workflow in the current role already satisfies every stage.

## How to read this machine

The board is a working model, not a dashboard. A seeded, deterministic simulation drives every machine. Its rates, counters and faults illustrate mechanisms; they are not telemetry from any real system and not measurements of anyone's work.

No machine depicts a real system, an employer's included. Each is a generic model of a mechanism that the attached records discuss.

Every record states its evidence class in words, and the four classes are never merged:

- **Production**: demonstrated in production.
- **Operational exposure**: real exposure, without enough evidence yet for broad ownership claims.
- **Developing**: a development target, not yet an established strength.
- **Independent**: independent engineering and research, kept separate from production experience.

The Archive shelves records by class, so calibration is spatial: a record's shelf is its claim.

A record that states a number, a scale or an outcome carries a claim boundary: what it asserts, and what it does not.

Independent projects are placed by the question each addresses. Placement does not imply dependencies between them, maturity or production use.

Every panel reads in the same order: what is happening, why it matters, the evidence, then the mechanism.
