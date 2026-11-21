+++
slug = "repave-observer-not-waiting"
title = "A repave process reported as hung"
kind = "case-study"
evidence = "production"
machines = ["lab.workers", "lab.fsm", "obs.normalize"]
tags = ["python", "linux"]
flagship = false
summary = "A rebuild workflow reported as failing had completed downstream; the supervising Python process was blocked on child-process output it captured but never read. A minimal reproduction and I/O tracing proved the mechanism, and output draining and observation changes followed."
claim_boundary = "One production incident. It is not evidence of broad concurrent-systems experience, and it does not claim that the repave workflow as a whole is now re-entrant or idempotent."
+++

## Result

A rebuild workflow reported as broadly failing was shown not to be failing where it appeared to fail. The downstream rebuild API had completed, while the supervising Python process went on waiting long afterwards and then dropped hosts from the workflow. The blocking mechanism was in the observer: output from a child process was captured but not consumed.

## Why it matters

The reported symptom named the wrong mechanism. Taken at face value, it points at the hosts or at the rebuild API, yet the rebuild API had completed. The fault was a disagreement between two observers of the same work: the downstream system's completion signal and the supervisor's local state.

This is the distinction the rest of this site is built around. A request that was accepted, or a process that is still waiting, is not evidence of the system's state. The state has to be observed independently.

## Evidence

- Compared the downstream execution signal with the observer's behavior, rather than accepting the reported host failure as the root cause.
- Inspected the process handling and found captured child-process output that nothing read.
- Built a minimal reproduction that filled the operating-system pipe buffer and reproduced the blocking behavior outside the production workflow.
- Added I/O tracing so the mechanism is observable rather than inferred.
- Implemented output draining and observation changes.
- Documented the incident and presented the root-cause analysis.

What the incident demonstrates: cross-layer debugging, falsifying an initial failure classification, Python subprocess and I/O reasoning, minimal reproduction as evidence, observability inside automation, and technical communication through a root-cause analysis.

## Mechanism

A pipe between two processes has a fixed kernel buffer. A process that writes to a full pipe blocks until something reads from the other end. Output that is captured but not consumed fills that buffer, and the child blocks on its next write. Anything waiting on the child waits with it. From outside, that looks like a hung rebuild.

Reading the output while waiting removes the block. Tracing the I/O makes the condition visible when it occurs, instead of leaving it to be inferred from a timeout.
