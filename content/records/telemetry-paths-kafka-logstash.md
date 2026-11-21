+++
slug = "telemetry-paths-kafka-logstash"
title = "Telemetry paths: Kafka, Logstash and Splunk ingestion"
kind = "note"
evidence = "production"
machines = ["obs.intake.*"]
tags = ["kafka", "logstash", "splunk", "salt"]
flagship = false
summary = "Production troubleshooting and configuration along logging and telemetry paths: Kafka, Logstash, Splunk ingestion, Salt execution returns and platform telemetry."
claim_boundary = "Troubleshooting and configuration on these paths. The record does not claim design or ownership of the Kafka or Logstash infrastructure."
+++

## Result

Production work covers troubleshooting and configuration along the telemetry path, from Kafka and Logstash through Splunk ingestion, including Salt execution returns and platform telemetry.

## Why it matters

Data missing at the end of a pipeline says little about where it was lost. Following the path stage by stage separates a source that did not emit from a transport that did not deliver and an ingest step that did not index.

## Evidence

Production troubleshooting and configuration around:

- Kafka;
- Logstash;
- Splunk ingestion;
- Salt execution returns;
- platform telemetry.

## Mechanism

The Observability bay on this board models a telemetry path of the same shape: intake ports, normalization, batching, storage, indexing and query. It is a generic model of the mechanism, not a diagram of a production system.
