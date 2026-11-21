+++
slug = "reentrancy-and-interruption-safety"
title = "Re-entrancy, idempotence and interruption safety"
kind = "direction"
evidence = "developing"
machines = ["lab.fsm"]
tags = ["systems"]
flagship = false
summary = "A development target: model rebuild lifecycle states explicitly, separate safe observation from mutating transitions, and make duplicate or stale requests reject, resume or converge by design. Not an established production strength."
claim_boundary = "A direction, not a result. No workflow is claimed to be re-entrant, idempotent or safe to resubmit."
+++

## Target

A state-transition test matrix showing the rebuild lifecycle's behavior for fresh, running, partially failed, completed and stale executions.

## Why it matters

A rebuild that is invoked twice, or resumed after an interruption, should do one of three things on purpose: refuse, continue, or converge on the state already reached. It should not repeat a mutation by accident. The production rebuild workflows in the lifecycle domain are not claimed to have these properties today; this direction is the work that would change that.

## Evidence

Class: developing. There is no result to report yet. The related production evidence is the [repave observer case](/archive/repave-observer-not-waiting), which changed how a rebuild is observed, not how it behaves on re-entry.

## Mechanism

- Model the rebuild lifecycle's states explicitly.
- Distinguish safe observation from a mutating transition.
- Make duplicate or stale requests reject, resume or converge intentionally.
- Test interruption at each state boundary.

The state drum in the Systems Lab bay demonstrates the idea with the same five states: fresh, running, partial, complete and stale. An illegal transition is rejected rather than applied. The drum is a demonstration, not the production implementation.
