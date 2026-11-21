+++
slug = "evidence-and-claims"
title = "Evidence Classes and Claim Boundaries"
category = "Methodology"
updated = "2026-11-21"
summary = "Why career records and open-source projects on kennedy.mosoti.dev separate production ownership, operational exposure, developing goals, and independent engineering with explicit claim boundaries."
tags = ["methodology", "reliability", "claims"]
+++

Technical writing routinely blurs production ownership, exposure, future aspirations, and side projects into uniform claims. This site enforces strict separation across both career records and software showcases.

## Career Evidence Classes

| Class | Meaning | Example |
| --- | --- | --- |
| **Production** | Demonstrated inside a defined operational domain with observable results | [Splunk forwarder convergence](/archive/forwarder-convergence-least-privilege) |
| **Operational exposure** | Real participation in workflows without claiming architectural ownership | [Operational exposure](/archive/operational-exposure) |
| **Developing** | Explicit engineering targets and rules not yet claimed as mature practice | [Re-entrancy & interruption safety](/archive/reentrancy-and-interruption-safety) |
| **Independent** | Self-directed systems research kept completely separate from employer work | [Metron](/archive/metron), [FabricO11y](/archive/fabrico11y) |

## Project Lifecycle Classes

Open-source projects use three honest lifecycle tags rather than inflating unverified experiments:

- **`Released Alpha`**: Tagged, installable packages with documented release notes and verified test suites ([Nomos](/projects/nomos), [FabricO11y](/projects/fabrico11y)).
- **`In Progress`**: Active implementation toward initial alpha milestones ([Metron](/projects/metron), [Jacquard](/projects/jacquard)).
- **`Architecture`**: Design models and extracted core crates under active verification ([Hephaestus](/projects/hephaestus)).

## Automated Guardrails

Automated Rust tests run on every change to verify that every case study and numerical claim carries a `claim_boundary`, production skills link exclusively to production records, and zero private client identifiers or internal IP/credential material ever enter public artifacts.
