+++
slug = "recursive-hexagonal-architecture"
title = "Recursive Hexagonal Architecture (RHA)"
kind = "project"
evidence = "independent"
machines = ["lattice.scheduler", "archive.station"]
tags = ["architecture", "agents"]
flagship = true
summary = "Independent, evidence-oriented engineering research on contribution semantics, testable architecture boundaries, agent and human collaboration, verification, falsifiable claims and provenance. Its research spec is a research artifact, not proof that the architecture is superior."
claim_boundary = "Independent research. No claim that the architecture outperforms alternatives, or that it has been adopted in production."
+++

## Aim

An engineering research project about architecture as a claim that can be tested. The questions are what a contribution to a system means, where its boundaries lie, how those boundaries are tested, and how a claim about the system can be falsified and traced to its source.

## Why it matters

When agents and humans both contribute to a codebase, it is not enough to know that a change works. It also matters what the change claims, who or what made the claim, and how the claim could be shown to be wrong. Architecture boundaries that can be tested turn those questions into checks.

## Evidence

Class: independent. RHA is independent research, kept separate from production experience.

The project has a research spec. It is treated as a research artifact, not as evidence that the architecture is better than the alternatives. Whether the approach holds up is the open question the research exists to test.

## Mechanism

The research is organized around six concerns:

- contribution semantics;
- testable architecture boundaries;
- agent and human collaboration;
- verification;
- falsifiable claims;
- provenance.

On this board, the Lattice scheduler assigns work, and the Archive reading station opens each record with its evidence class and claim boundary. Both illustrate the concerns above; neither is an implementation of RHA.
