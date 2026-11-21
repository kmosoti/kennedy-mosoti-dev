+++
slug = "operational-exposure"
title = "Operational exposure"
kind = "note"
evidence = "operational-exposure"
machines = ["infra.quarantine", "lab.breaker"]
tags = ["operations"]
flagship = false
summary = "Exposure to incident response, weekend operational support, high-availability and disaster-recovery workflows, production restoration, SRE collaboration, cluster health and lifecycle operations, and recurring-failure remediation. Real exposure, without enough evidence yet for broad ownership claims."
claim_boundary = "Exposure, not ownership. The record does not claim incident leadership or incident command, end-to-end SRE ownership, or ownership of high-availability and disaster-recovery design."
+++

## Result

Participation in the operational work around the lifecycle domain, at a level of evidence that supports exposure but not broad ownership.

## Why it matters

Exposure and ownership are different claims. Keeping them apart is the point of this record: the work below is real, and it is not presented as leadership of it.

The escalation role for repave and patch-related failures, after initial SRE triage and restoration, is production evidence and is recorded on the [role](/archive/netbuilder-observability-engineer), not here.

## Evidence

- incident response;
- weekend operational support;
- high-availability and disaster-recovery workflows;
- production restoration;
- SRE collaboration;
- cluster health and lifecycle operations;
- recurring-failure remediation.

## Mechanism

Initial triage and restoration are performed by SRE. Failures in the repave and patch domain then reach this work for later-stage investigation. Moving from restoring and escalating recurring failures toward preventing their recurrence is a development direction: [Incident closure](/archive/incident-closure).
