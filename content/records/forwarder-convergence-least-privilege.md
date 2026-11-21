+++
slug = "forwarder-convergence-least-privilege"
title = "Splunk forwarder convergence across a large RHEL fleet"
kind = "case-study"
evidence = "production"
machines = ["infra.config", "infra.rollout", "infra.desired"]
tags = ["salt", "linux", "splunk"]
flagship = false
summary = "After host rebuilds, a monolithic Salt configuration path could fail to converge, and a required service-management operation failed for want of privilege. The configuration was decomposed into composable Salt states, the operation received only the privilege it needs rather than broad root, and rollout was staged from a single host to production."
claim_boundary = "\"Large\" is the only scale claimed; no host count is stated. The staged rollout is evidence of change discipline. The record does not describe the wider safeguards around fleet changes, and individual judgment is not claimed as the only safeguard."
+++

## Result

The Salt configuration for the Splunk forwarder was restructured from one monolithic path into more composable states. A service-management operation that failed for want of privilege was fixed by granting the least privilege that operation required, not broad root. The change reached production through a staged rollout.

## Why it matters

A privileged operation that fails can be made to succeed in two ways: grant broad root, or grant exactly the operation. Both make the failure go away. Only the second keeps authority close to what the task needs. On a fleet, the difference is repeated on every host the change reaches.

Decomposition changes what a failure means. A smaller state that fails points at a smaller step, and can be ordered, required and re-run on its own.

## Evidence

- Identified the convergence problem.
- Restructured the monolithic Salt configuration into more composable states.
- Introduced the least privilege necessary for the required service-management operation, instead of broad root access.
- Staged the rollout: a single host, then a cluster, then a pre-production stage, then production.

What the work demonstrates: Salt state design, decomposition of monolithic configuration, desired-state reasoning, Linux service management, least-privilege judgment, staged rollout at fleet scale, and production change discipline.

## Mechanism

Salt applies desired state. Each state declares what should be true of a host, and a run moves the host toward it. A monolithic state couples steps that can fail independently. Composable states let each step be expressed, ordered and verified on its own.

The failing step needed one service-management operation that could not be performed without privilege. The remediation granted that operation and nothing wider.

Each rollout stage widened the set of hosts the change reached: one host, one cluster, a pre-production stage, production. Changes made through fleet configuration management carry fleet-wide reach, so the order of widening is part of the change.
