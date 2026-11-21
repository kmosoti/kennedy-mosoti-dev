+++
slug = "netbuilder-observability-engineer"
title = "Associate Observability Engineer, NETbuilder"
kind = "role"
evidence = "production"
machines = ["core.registers", "infra.slots", "obs.store"]
tags = ["python", "salt", "linux", "splunk"]
flagship = false
period = "October 2024 – present"
summary = "Functionally an infrastructure and platform automation engineer with operational ownership of a fleet lifecycle domain: rebuild automation, SaltStack desired state, Splunk platform lifecycle, Linux/RHEL configuration, patch-posture automation, telemetry around automation behavior, and escalation investigation after SRE restores service."
claim_boundary = "Meaningful autonomy inside a defined production domain. Several architectural constraints and incident responsibilities are inherited or owned elsewhere. The record does not claim end-to-end SRE ownership, incident leadership or SLO ownership."
+++

## Result

The official title understates the shape of the work. Functionally, the role is infrastructure and platform automation with operational ownership of a fleet lifecycle domain on a large RHEL and Splunk estate. The employer is NETbuilder.

## Why it matters

The production evidence on this site comes from this role, so its scope sets what the rest of the site can claim. Inside the domain, the work includes:

- identifying some production problems independently;
- deriving and implementing solutions inside the existing platform;
- owning a defined operational domain after initial incident restoration;
- making changes with fleet-scale consequences;
- performing root-cause analysis;
- influencing team operating practice.

## Evidence

Case studies:

- [A repave process reported as hung](/archive/repave-observer-not-waiting)
- [Splunk forwarder convergence across a large RHEL fleet](/archive/forwarder-convergence-least-privilege)
- [Patch-posture visibility and rebuild cadence](/archive/patch-posture-cadence)
- [Rejecting unnecessary AI-generated infrastructure code](/archive/deleting-generated-infrastructure-code)

Notes:

- [Search-filter automation across Splunk roles](/archive/splunk-role-search-filters)
- [Configuration for a Splunk topology expansion](/archive/splunk-topology-expansion)
- [Dashboard automation over an undocumented JSON surface](/archive/dashboard-json-validation)
- [Telemetry paths: Kafka, Logstash and Splunk ingestion](/archive/telemetry-paths-kafka-logstash)
- [SELinux labeling as desired state](/archive/selinux-desired-state)

Wider operational work, where the evidence is exposure rather than ownership, is recorded separately: [Operational exposure](/archive/operational-exposure).

## Mechanism

The work in the domain:

- repave and rebuild automation;
- SaltStack desired-state work;
- Splunk platform lifecycle operations;
- Linux and RHEL configuration;
- patch-posture automation and reporting;
- telemetry around automation behavior;
- escalation investigation after SRE restores service;
- configuration and lifecycle failure analysis.

As an escalation owner within the domain, the work combines Salt, Linux, platform telemetry and API evidence to distinguish workflow, host, configuration and downstream-state failures.
