# Documentation Versioning Strategy

**Version:** 1.0  
**Status:** Active  
**Effective Date:** 2026-01-19  

## Overview

This document defines the versioning strategy for Markenz documentation, ensuring alignment with software releases while maintaining clarity for users consuming documentation through different channels.

## Versioning Philosophy

Documentation is treated as a product with its own lifecycle, but it remains tightly coupled to the software it documents. This strategy balances the need for documentation stability with the requirement for accuracy across software versions.

## Version Numbering Scheme

### Format: `v{major}.{minor}.{patch}`

- **Major Version:** Significant restructuring, breaking changes in content organization, or deprecation of major documentation sections
- **Minor Version:** New feature documentation, additional guides, non-breaking organizational improvements
- **Patch Version:** Error corrections, clarification updates, minor content additions

### Alignment with Software Versions

Documentation versions track software versions with the following mapping:

| Software Version | Documentation Version | Relationship |
|------------------|----------------------|---------------|
| 2.0.x | v2.0.x | Direct alignment |
| 2.1.x | v2.1.x | Feature documentation updates |
| 2.0.x | v2.0.x | Patch corrections only |

## Directory Structure

```
/docs/
├── v2/                    # Current major version
│   ├── architecture/      # Core architecture documentation
│   ├── api/              # API reference documentation
│   ├── guides/           # User guides and tutorials
│   ├── operations/       # Operations and deployment
│   └── README.md         # Version index and navigation
├── v1/                    # Previous major version (maintenance mode)
│   ├── [same structure as v2]
│   └── README.md
└── archived/              # Historical versions (read-only)
    ├── v0.9/
    └── v0.8/
```

## Version Support Policy

### Current Version (v2)
- **Status:** Active development
- **Support:** Full updates, new content, corrections
- **Timeline:** Until next major version release

### Previous Major Version (v1)
- **Status:** Maintenance mode
- **Support:** Security updates and critical corrections only
- **Timeline:** 12 months after v2 release

### Archived Versions
- **Status:** Read-only
- **Support:** No updates
- **Purpose:** Historical reference and compliance requirements

## Release Process

### Pre-Release Validation
1. **Version Alignment Check:** Verify documentation version matches software version
2. **Content Audit:** Ensure all new features are documented
3. **Breaking Changes Review:** Identify and document breaking changes
4. **Link Validation:** Verify all internal and external links
5. **Code Example Testing:** Validate all code snippets

### Release Activities
1. **Version Tagging:** Create appropriate version tags in repository
2. **Directory Updates:** Update version-specific directories
3. **Navigation Updates:** Update cross-version navigation
4. **Changelog Updates:** Document all documentation changes
5. **Distribution:** Update all publishing channels

### Post-Release Monitoring
1. **User Feedback Collection:** Monitor feedback channels
2. **Accuracy Verification:** Track reported inaccuracies
3. **Usage Analytics:** Monitor documentation engagement
4. **Issue Tracking:** Address documentation issues promptly

## Migration Guidelines

### For Users Upgrading Documentation

When upgrading from one documentation version to another:

1. **Review Breaking Changes:** Check the documentation changelog for breaking changes
2. **Update Bookmarks:** Update bookmarks to new version URLs
3. **Verify Integrations:** Ensure tool integrations point to correct version
4. **Review New Features:** Familiarize with newly documented features

### For Maintainers

When creating new documentation versions:

1. **Create Version Directory:** Establish new version directory structure
2. **Copy Current Content:** Start with current version as baseline
3. **Apply Breaking Changes:** Implement necessary structural changes
4. **Update References:** Update all cross-references and links
5. **Archive Old Version:** Move previous version to appropriate location

## URL Strategy

### Version-Specific URLs
- Current version: `/docs/v2/section/page`
- Previous version: `/docs/v1/section/page`
- Archived versions: `/docs/archived/v0.9/section/page`

### Default Version Resolution
- `/docs/` redirects to current stable version
- `/docs/latest` always points to most recent version
- `/docs/stable` points to current stable major version

## Cross-Version Compatibility

### Content Compatibility
- **Compatible Content:** Core concepts, fundamental architecture
- **Version-Specific Content:** API references, configuration examples
- **Deprecated Content:** Clearly marked with migration guidance

### Link Management
- **Internal Links:** Use relative paths within version
- **Cross-Version Links:** Explicit version specification required
- **External Links:** Regular validation and maintenance

## Quality Assurance

### Automated Checks
- Link validation across all versions
- Markdown formatting consistency
- Code example compilation testing
- Image and diagram rendering verification

### Manual Reviews
- Content accuracy verification
- User experience assessment
- Navigation flow testing
- Accessibility compliance checking

## Communication Strategy

### Version Announcements
- Release notes with documentation changes
- Migration guides for breaking changes
- Deprecation notices with timelines
- New feature documentation highlights

### Ongoing Communication
- Regular documentation quality reports
- User feedback summaries
- Improvement roadmap updates
- Best practice recommendations

---

**Authority:** This strategy is maintained by the Documentation Product Owner and requires review for any modifications affecting version numbering or support policies.
