+++
slug = "incident-closure"
title = "Incident closure"
kind = "direction"
evidence = "developing"
machines = ["lab.breaker"]
tags = ["operations"]
flagship = false
summary = "A development target: in a domain Kennedy owns, move from restoring and escalating recurring failures toward preventing their recurrence."
claim_boundary = "A direction. Recurring-failure remediation is currently operational exposure, and no failure class is claimed closed."
+++

## Target

For one recurring failure class, a complete chain: reproduction, root cause, source fix, regression test or guard, and a monitored postcondition.

## Why it matters

Restoring service ends an incident. It does not end the failure class that caused it. A class is closed only when the fix is in the source, a test or guard stops it returning unnoticed, and a monitored postcondition shows it has not returned.

## Evidence

Class: developing. Today the evidence for recurring-failure remediation is [operational exposure](/archive/operational-exposure), not ownership.

## Mechanism

1. Reproduce the failure.
2. Establish the root cause.
3. Fix it at the source.
4. Add a regression test or guard.
5. Monitor the postcondition.

The breaker in the Systems Lab bay trips after repeated faults and resets after a delay. It demonstrates containment, the step before closure.
