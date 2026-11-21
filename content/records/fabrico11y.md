+++
slug = "fabrico11y"
title = "FabricO11y: a local observability engine"
kind = "project"
evidence = "independent"
machines = ["obs.batcher", "obs.store", "obs.indexer"]
tags = ["observability", "arrow", "parquet"]
flagship = true
summary = "Independent systems and data project: a local observability engine spanning ingestion, write-ahead logging, Arrow and Parquet representation, query execution, indexing, approximate data structures, live and historical execution, and operational telemetry. A working vertical slice takes priority over architectural breadth."
claim_boundary = "Independent engineering, not production experience. The scope lists what the project covers, not what is complete. No production use of Arrow or Parquet is claimed."
+++

## Aim

A local observability engine, built as a systems and data project. The stated priority is a working vertical slice over architectural breadth: one path from ingestion to query that runs, before many paths that do not.

## Why it matters

The production records on this site troubleshoot and configure telemetry paths through Kafka, Logstash and Splunk ingestion, and automate and operate the Splunk platform. That work meets these systems from the outside. FabricO11y's scope is the inside: how data is made durable, how it is represented, how it is found again, and how an answer over live data differs from an answer over history.

## Evidence

Class: independent. FabricO11y is independent engineering, kept separate from the production telemetry work. Arrow and Parquet designs are independent work only.

The project's scope:

- ingestion;
- write-ahead logging;
- Arrow and Parquet representation;
- query execution;
- indexing;
- approximate data structures;
- live versus historical execution;
- operational telemetry.

This record does not claim which parts are complete; the project's own repository owns its status.

## Mechanism

The Observability bay on this board runs a pipeline of the same shape. A batcher accumulates packets and flushes them as one batch. Each batch lands on a shelf as a segment. An indexer traverses each newly landed segment, and queries fan out to the indexed segments and return a result.

The bay is a generic model of those stages, driven by a seeded simulation. It is not FabricO11y's implementation, and its numbers are not FabricO11y's measurements.
