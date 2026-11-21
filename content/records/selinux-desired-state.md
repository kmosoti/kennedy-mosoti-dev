+++
slug = "selinux-desired-state"
title = "SELinux labeling as desired state"
kind = "note"
evidence = "production"
machines = ["infra.config"]
tags = ["linux", "selinux"]
flagship = false
summary = "Worked with explicit SELinux filesystem labeling and restoration behavior, treating permissions as declared state rather than ad-hoc shell mutation."
claim_boundary = "An approach applied in production Linux configuration work. The record does not claim SELinux policy authorship or ownership of how SELinux is configured."
+++

## Result

SELinux filesystem labels are handled as explicit labeling and restoration behavior, not as one-off changes made from a shell.

## Why it matters

A label changed by hand on one host is invisible to the next person and does not survive a relabel or a rebuild. A declared label is reapplied every time the configuration is.

## Evidence

- Worked with explicit SELinux filesystem labeling and restoration behavior rather than treating permissions as ad-hoc shell mutation.

## Mechanism

Declare the expected context, apply it, and restore it with the system's own tools, so the state is the same after a rebuild as before it.
