+++
slug = "dashboard-json-validation"
title = "Dashboard automation over an undocumented JSON surface"
kind = "note"
evidence = "production"
machines = ["obs.indexer"]
tags = ["splunk", "telemetry"]
flagship = false
summary = "Built dashboard and reporting automation around an undocumented dashboard JSON surface, with validation rules and repeatable publication from saved telemetry searches."
claim_boundary = "The record does not claim that the dashboard format is now documented, or that the validation covers every error the format allows."
+++

## Result

Dashboards and reports are produced by automation from saved telemetry searches, over a dashboard JSON surface with no published documentation. The automation includes validation behavior: rules inferred from the format, enforced as checks.

## Why it matters

An undocumented format has no contract. Its rules have to be inferred from examples and enforced by whoever produces it. Validation writes the inferred rules down as checks, so a malformed definition is caught by a rule rather than discovered by a reader.

## Evidence

- Built automation around an undocumented dashboard JSON surface.
- Developed validation behavior, including an implementation of explicit validation rules.
- Repeatable publication from saved telemetry searches.

## Mechanism

Saved telemetry searches are the source of what is published. The validation rules stand in for the documentation the format does not have.
