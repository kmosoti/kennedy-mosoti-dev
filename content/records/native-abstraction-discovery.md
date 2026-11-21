+++
slug = "native-abstraction-discovery"
title = "Native-abstraction discovery before implementation"
kind = "direction"
evidence = "developing"
machines = ["lattice.verifier"]
tags = ["salt", "python", "linux"]
flagship = false
summary = "A development target: make \"what already solves this?\" a required design step, comparing Salt-native, Python standard-library, Linux-native and repository-native options before any custom implementation."
claim_boundary = "A development target, not an established practice. The record does not claim that design notes already record rejected alternatives for every consequential change."
+++

## Target

Design notes for consequential changes include the alternatives considered and why each was rejected.

## Why it matters

The [generated-code case](/archive/deleting-generated-infrastructure-code) showed custom code arriving before an existing Salt capability was found. Code that duplicates a native primitive adds code to review and maintain, and no function; in that case it also added blast radius.

## Evidence

Class: developing. The motivating case is production evidence; the practice itself is not yet established.

## Mechanism

Before a custom implementation, exhaust:

1. native platform primitives (Salt, Linux, Splunk);
2. standard-library behavior;
3. maintained existing abstractions;
4. existing repository conventions.

Custom code is the last option, and the design note records what was tried and why it was rejected.
