+++
slug = "deleting-generated-infrastructure-code"
title = "Rejecting unnecessary AI-generated infrastructure code"
kind = "case-study"
evidence = "production"
machines = ["lattice.verifier"]
tags = ["salt", "linux", "ai"]
flagship = false
summary = "AI-assisted development produced custom virtual-environment and ownership-convergence code that duplicated capabilities already present in Salt and recursively changed ownership around symlink-sensitive filesystem state. After peer review, the custom implementation was removed in favor of Salt-native behavior."
claim_boundary = "Evidence of deleting generated code and recognizing blast radius. It does not show that the AI-assisted development process is mature. It shows that generated implementation can outrun the discovery of existing abstractions, and that a native-capability check before implementation should become standard practice."
+++

## Result

Custom infrastructure code produced with AI assistance was removed and replaced with Salt-native behavior. The removed code reimplemented virtual-environment and ownership convergence that Salt already expresses, and it included recursive ownership changes around symlink-sensitive filesystem state.

## Why it matters

The custom code duplicated an existing capability, so it added risk without adding function. The risk was specific: recursive ownership changes around paths that may be symbolic links can reach further than the tree they were meant for, and a configuration state runs on every host it targets.

It also matters because the cost of writing code is no longer a reliable signal of whether the code should exist.

## Evidence

- The generated implementation duplicated capabilities already present in Salt.
- It included dangerous recursive ownership mutation around symlink-sensitive filesystem state.
- The design was updated after peer review: the custom implementation was removed in favor of Salt-native behavior.

What the case demonstrates: willingness to delete generated code, recognition of blast radius, a preference for native platform primitives once they are discovered, and updating a design after peer review.

What it does not demonstrate: a mature AI-assisted development process. The case is evidence of a gap, and of the response to it, not of a finished practice.

## Mechanism

The failure was one of ordering, not of tooling. AI assistance reduces implementation friction enough that custom code can arrive before the native primitives, the standard library, maintained abstractions and the repository's own conventions have been searched. That is a workflow problem, not evidence that AI assistance is inherently harmful.

Two responses follow, and both are recorded under the developing class rather than as established practice: [a gate of eight questions](/archive/ai-generated-code-gate) before accepting AI-generated infrastructure code, and [a native-capability check](/archive/native-abstraction-discovery) as a required design step.
