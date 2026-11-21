+++
# Production: profile §14 "Safe claims". Independent: the items of §2
# "Independent engineering / research" that §14's "Independent-only unless
# evidence changes" list names; that list restricts claims, so an item on it
# with no recorded independent work is not a skill (see README.md, "Open for
# owner review"). A skill cites only records of its own evidence class, so
# classes are never blended. An empty `records` list means no record on the
# site supports the skill directly.

[[skills]]
tag = "python"
label = "Python automation"
evidence = "production"
records = ["repave-observer-not-waiting", "netbuilder-observability-engineer"]

[[skills]]
tag = "salt"
label = "Salt automation"
evidence = "production"
records = ["forwarder-convergence-least-privilege", "deleting-generated-infrastructure-code", "netbuilder-observability-engineer"]

[[skills]]
tag = "linux"
label = "Large RHEL estate"
evidence = "production"
records = ["forwarder-convergence-least-privilege", "selinux-desired-state", "netbuilder-observability-engineer"]
boundary = "\"Large\" is the only scale claimed; no host count is stated."

[[skills]]
tag = "splunk"
label = "Large Splunk estate"
evidence = "production"
records = ["splunk-topology-expansion", "splunk-role-search-filters", "forwarder-convergence-least-privilege", "dashboard-json-validation", "telemetry-paths-kafka-logstash"]
boundary = "\"Large\" is the only scale claimed; no node or role count is stated. Platform automation and operations, not ownership of the Splunk architecture."

[[skills]]
tag = "configuration"
label = "Fleet-scale configuration changes"
evidence = "production"
records = ["forwarder-convergence-least-privilege", "splunk-topology-expansion"]

[[skills]]
tag = "rca"
label = "Operational root-cause analysis"
evidence = "production"
records = ["repave-observer-not-waiting", "netbuilder-observability-engineer"]

[[skills]]
tag = "rollout"
label = "Staged production rollout"
evidence = "production"
records = ["forwarder-convergence-least-privilege"]

[[skills]]
tag = "least-privilege"
label = "Least-privilege remediation"
evidence = "production"
records = ["forwarder-convergence-least-privilege"]

[[skills]]
tag = "telemetry"
label = "Dashboard and telemetry automation"
evidence = "production"
records = ["dashboard-json-validation", "telemetry-paths-kafka-logstash", "patch-posture-cadence"]

[[skills]]
tag = "lifecycle"
label = "Infrastructure lifecycle support"
evidence = "production"
records = ["netbuilder-observability-engineer", "patch-posture-cadence", "repave-observer-not-waiting"]

[[skills]]
tag = "rust"
label = "Rust systems work"
evidence = "independent"
records = []
boundary = "Independent work only; production Rust is not claimed."

[[skills]]
tag = "arrow"
label = "DataFusion, Arrow and Parquet designs"
evidence = "independent"
records = ["fabrico11y"]
boundary = "Independent work only; production DataFusion, Arrow or Parquet is not claimed. FabricO11y's scope covers the Arrow and Parquet representation."

[[skills]]
tag = "agent-runtime"
label = "Agent-runtime reliability research"
evidence = "independent"
records = ["metron"]
boundary = "Independent research only; production ownership of an agent runtime is not claimed."
+++

## Production

Demonstrated in production. Each skill links to the records that support it.

- Python automation
- Salt automation
- Large RHEL estate ("large" is the only scale claimed)
- Large Splunk estate
- Fleet-scale configuration changes
- Operational root-cause analysis
- Staged production rollout
- Least-privilege remediation
- Dashboard and telemetry automation
- Infrastructure lifecycle support

## Independent only

Independent engineering and research. None of these is claimed as production experience.

- Rust systems work
- DataFusion, Arrow and Parquet designs
- Agent-runtime reliability research
