+++
slug = "slo-reasoning"
title = "Service Level Objective reasoning"
kind = "direction"
evidence = "developing"
machines = ["obs.query"]
tags = ["reliability"]
flagship = false
summary = "A development target: connect infrastructure automation to user-visible or platform-visible reliability objectives, and learn when availability, latency, freshness or completion correctness is the meaningful service indicator."
claim_boundary = "A direction. No Service Level Objective or error-budget ownership is claimed."
+++

## Target

One real service or workflow with explicit Service Level Indicators (SLIs), a Service Level Objective (SLO), and a documented reliability tradeoff.

## Why it matters

Automation is easy to measure by whether it ran. The people who depend on it care whether the outcome was correct and on time. An SLO states which of those matters, how much, and what is traded for it.

## Evidence

Class: developing. There is no SLO or error-budget ownership to report.

## Mechanism

- Choose the indicator that reflects the service: availability, latency, freshness or completion correctness.
- Set an objective against it.
- Record the reliability tradeoff the objective implies.

The Observability bay distinguishes records acknowledged at intake from records a query can find. That difference is the kind of indicator this direction is about.
