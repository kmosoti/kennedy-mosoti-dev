+++
slug = "observing-system-state-not-exit-codes"
title = "Observing System State, Not Exit Codes"
date = "2026-11-12"
author = "Kennedy Mosoti"
summary = "Why automation reliability begins by trusting independent postcondition signals over local return codes or synchronous request acks."
tags = ["nomos", "fabrico11y", "observability", "linux"]
+++

Every platform engineer eventually meets a workflow that returned `0`—or hung indefinitely after succeeding—because the automation trusted its own wrapper state instead of observing the target machine.

## Three Concrete Mechanisms

1. **Kernel Pipe Deadlocks in Supervisors:** As documented in [A repave process reported as hung](/archive/repave-observer-not-waiting), capturing child-process `stdout`/`stderr` without draining the pipe buffer freezes the child even when the remote rebuild API already finished.
2. **Desired State vs. Imperative Scripts:** In [Splunk forwarder convergence across a large RHEL fleet](/archive/forwarder-convergence-least-privilege), decomposing monolithic Salt state into composable desired-state declarations with least-privilege scoping turned host rebuild drift into targeted, re-runnable convergence steps.
3. **Intake Acceptance vs. Canonical Custody:** In [FabricO11y](/projects/fabrico11y), admitting telemetry events into a local buffer is distinct from advancing Spindle's custody ledger and canonical query watermark.

## How Nomos Models the Loop

In [Nomos](/projects/nomos), a reconciliation step is never declared complete when a command finishes; completion requires re-running **Substrate** probes against `/etc`, `systemd`, and `sysctl` and confirming that **Variance** against the **Canon** has dropped to zero.

Good observability is not a dashboard added after failure—it is the mechanism that keeps automation honest while changes are running.
