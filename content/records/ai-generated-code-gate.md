+++
slug = "ai-generated-code-gate"
title = "A gate for AI-generated infrastructure code"
kind = "note"
evidence = "developing"
machines = ["lattice.verifier"]
tags = ["ai"]
flagship = false
summary = "Eight questions to answer before accepting AI-generated infrastructure code, from the invariant it should establish to the observation that proves convergence. AI-generated code is a proposal; acceptance remains an engineering decision."
claim_boundary = "A stated rule and a development target, not an established practice. It is not claimed to be applied consistently or to have a measured effect."
+++

## Rule

AI-generated code is a proposal. Acceptance remains an engineering decision. Before accepting AI-generated infrastructure code, answer:

1. What invariant is the change meant to establish?
2. Which native Salt, Linux, Python or Splunk primitive already expresses it?
3. Why is custom code necessary?
4. What authority does the proposed implementation exercise?
5. What is the blast radius if the target or the assumption is wrong?
6. What happens on retry, interruption, partial success and duplicate execution?
7. What observation proves convergence?
8. What simpler implementation was tested and rejected?

## Why it matters

AI assistance reduces implementation friction enough that custom code can arrive before native platform primitives, standard-library behavior, maintained abstractions and repository conventions have been exhausted. That is a workflow problem, not evidence that AI assistance is inherently harmful. The gate puts the search, and the questions of authority and failure, back in front of the code.

## Evidence

Class: developing. The gate follows the [generated-code case](/archive/deleting-generated-infrastructure-code), which is production evidence. AI is used for research, design exploration, implementation, testing ideas, counterexample generation and engineering analysis, and generated code has been rejected and deleted after architectural and safety problems were identified. Whether the gate is applied every time is not claimed.

## Mechanism

The questions follow the operating pattern. The first two model state and look for what already expresses it. Questions three to five constrain mutation: necessity, authority and blast radius. Question six covers recovery. Question seven is verification of convergence. Question eight is the evidence that a simpler path was tried.
