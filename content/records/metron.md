+++
slug = "metron"
title = "Metron: a bounded runtime for stochastic workers"
kind = "project"
evidence = "independent"
machines = ["lattice.cell", "lattice.verifier", "lattice.halt"]
tags = ["agents", "runtime"]
flagship = true
summary = "Independent work built around one thesis: stochastic generators may propose; a deterministic runtime owns authority, verification, state and commit."
claim_boundary = "Independent engineering and research, not production experience. The proof obligations listed are the standard the project is held to, not a statement of what is complete; no production use of an agent runtime is claimed."
+++

## Aim

A runtime in which the parts that guess are separated from the parts that decide. Stochastic generators may propose actions and artifacts. A deterministic runtime owns what they are allowed to do, verifies what they produce, holds the state, and decides what is committed.

## Why it matters

A stochastic worker's output is a sample, not a decision. If the same worker also holds the authority to act, every bad sample is an effect in the world. Moving authority, budgets, verification and commit into a deterministic runtime puts them where they can be reasoned about and tested.

The same rule appears at the level of practice in the [gate for AI-generated infrastructure code](/archive/ai-generated-code-gate): generated code is a proposal, and acceptance remains an engineering decision.

## Evidence

Class: independent. Metron is independent engineering and research, kept separate from production experience. Bounded runtimes for stochastic workers and capability-bounded AI runtime research are independent work.

The project's proof obligations:

- capability boundaries;
- resource budgets;
- effect mediation;
- commit and discard semantics;
- verifier failure modes;
- replay and provenance limits.

These define what the project has to demonstrate. This record does not claim which of them are met; the project's own repository owns its status.

## Mechanism

The Agent Lattice bay on this board illustrates the thesis. Each cell works under a budget piston that drains while it works. A cell whose budget reaches zero closes its shutter, and the scheduler routes around it. A completed cell emits an artifact that travels to a verifier, which accepts it or rejects it. Nothing a cell produces reaches the Archive without passing the verifier.

The bay is an illustration of the thesis, not Metron's implementation.
