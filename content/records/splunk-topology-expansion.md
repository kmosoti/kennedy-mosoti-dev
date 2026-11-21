+++
slug = "splunk-topology-expansion"
title = "Configuration for a Splunk topology expansion"
kind = "note"
evidence = "production"
machines = ["obs.store"]
tags = ["splunk"]
flagship = false
summary = "Delivered configuration changes associated with expanding a distributed Splunk deployment with additional indexers and search heads."
claim_boundary = "The record claims the configuration changes and states no node counts. It does not claim ownership of the capacity decision, the architecture or the expansion as a whole."
+++

## Result

The configuration changes for an expansion of a distributed Splunk deployment, with additional indexers and search heads, were delivered.

## Why it matters

In a distributed deployment, indexers store and search the data and search heads coordinate searches across them. New nodes join an estate that is already running, so their configuration has to agree with the nodes already there.

## Evidence

- Delivered configuration changes associated with the addition of indexers and search heads.

## Mechanism

Topology changes are configuration changes with estate-wide effect: the new nodes and the existing ones both have to be configured for the new shape of the deployment.
