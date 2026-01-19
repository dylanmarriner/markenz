# Architecture Decision Records (ADR)

**Status:** Active  
**Maintainer:** Principal Architect  
**Review Cycle:** As needed  

## Overview

This directory contains Architecture Decision Records (ADRs) for the Markenz project. ADRs capture important architectural decisions, along with their context, consequences, and rationale.

## ADR Process

### ADR Lifecycle

1. **Proposal:** ADR is drafted in "Proposed" status
2. **Review:** Technical review by architecture team
3. **Decision:** ADR is moved to "Accepted" or "Rejected"
4. **Implementation:** Decision is implemented in code/system
5. **Supersession:** ADR may be superseded by future decisions
6. **Deprecation:** ADR may be deprecated when no longer relevant

### ADR Status Values

- **Proposed:** Under consideration and review
- **Accepted:** Decision made and implemented
- **Rejected:** Decision not accepted
- **Deprecated:** No longer current but kept for historical reference
- **Superseded:** Replaced by a newer ADR

### ADR Template

All ADRs must follow the [ADR template](0000-adr-template.md).

## ADR Index

### Core Architecture Decisions

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [0001](0001-record-architecture-decisions.md) | Record Architecture Decisions | Accepted | 2026-01-19 |
| [0002](0002-docs-as-product-strategy.md) | Docs-as-a-Product Strategy | Accepted | 2026-01-19 |
| [0003](0003-deterministic-engine-architecture.md) | Deterministic Engine Architecture | Accepted | 2025-12-15 |
| [0004](0004-biological-agent-simulation.md) | Biological Agent Simulation Framework | Accepted | 2025-12-20 |
| [0005](0005-governance-layer-design.md) | Governance Layer and Physics of Law | Accepted | 2025-12-22 |

### Infrastructure and Operations

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [0006](0006-deployment-architecture.md) | Deployment Architecture and Containerization | Accepted | 2026-01-05 |
| [0007](0007-monitoring-and-observability.md) | Monitoring and Observability Strategy | Accepted | 2026-01-08 |
| [0008](0008-security-architecture.md) | Security Architecture and Access Control | Accepted | 2026-01-10 |

### Data and Storage

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [0009](0009-data-model-design.md) | Core Data Model Design | Accepted | 2026-01-12 |
| [0010](0010-serialization-format.md) | Serialization and Data Exchange Format | Accepted | 2026-01-15 |

### Performance and Scalability

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [0011](0011-performance-optimization.md) | Performance Optimization Strategy | Accepted | 2026-01-17 |
| [0012](0012-scalability-architecture.md) | Scalability and Distributed Simulation | Proposed | 2026-01-18 |

## Decision Categories

### Strategic Decisions
High-level decisions that affect the overall system direction and long-term architecture.

### Tactical Decisions
Specific implementation decisions that solve immediate problems while aligning with strategic direction.

### Operational Decisions
Decisions related to deployment, monitoring, maintenance, and operational procedures.

## Review Process

### Review Committee
- **Principal Architect:** Final approval authority
- **Technical Leads:** Subject matter expertise
- **Engineering Manager:** Resource and timeline considerations
- **Security Officer:** Security and compliance review

### Review Criteria
1. **Technical Soundness:** Decision is technically viable and well-reasoned
2. **Strategic Alignment:** Decision aligns with overall system goals
3. **Implementation Feasibility:** Decision can be implemented with available resources
4. **Risk Assessment:** Risks are identified and acceptable
5. **Documentation Quality:** ADR is complete and clearly written

### Review Timeline
- **Initial Review:** 5 business days from proposal
- **Final Decision:** 10 business days from proposal
- **Implementation:** As per project timeline

## ADR Maintenance

### Regular Reviews
- **Quarterly Review:** Review all active ADRs for continued relevance
- **Annual Audit:** Complete audit of ADR process and decisions
- **Version Alignment:** Ensure ADRs align with current system version

### Updates and Corrections
- **Minor Corrections:** Typos, formatting, clarification (no review required)
- **Substantive Changes:** Changes to decision or rationale (full review required)
- **Superseding:** New ADR supersedes existing decision (cross-reference required)

## Integration with Development

### Code References
- All implementations must reference relevant ADRs
- Code comments should include ADR numbers where applicable
- Pull requests should reference ADRs when implementing decisions

### Documentation Links
- Technical documentation should link to relevant ADRs
- Architecture diagrams should reference ADRs
- API documentation should include ADR references where appropriate

### Change Management
- Changes that affect ADR decisions must update the ADR
- New ADRs required for significant architectural changes
- ADR status changes must be communicated to development team

## Historical Archive

### Superseded ADRs
Older ADRs that have been superseded are kept for historical reference and to understand the evolution of the architecture.

### Rejected ADRs
Rejected ADRs are maintained to document considered alternatives and the reasoning for rejection.

### Deprecation Process
ADRs are deprecated when they are no longer relevant but removing them would lose valuable historical context.

## Tools

* **Template:** [0000-adr-template.md](0000-adr-template.md)
* **Process:** To propose a new decision, copy the template to `NNNN-title-of-decision.md` and open a Pull Request.

---

**ADR Process Authority:** This process is maintained by the Principal Architect and requires formal review for any modifications to the ADR process itself.
