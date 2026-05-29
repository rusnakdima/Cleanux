# Changelog

All notable changes to Cleanux will be documented in this file.

## [0.3.0] - 2026-05-26 - Refactor Release

### Added

- **Architecture**: Full backend restructuring with `dto.rs`, `state.rs`, `middleware.rs`
- **Security Module**: Path validation, allowlist enforcement, symlink protection, privilege operation logging
- **Frontend Architecture**: Lazy-loaded routes, signal-based stores, interceptors, resolvers, guards
- **Design System**: SCSS tokens for colors, typography, spacing
- **i18n**: 10 languages (en, es, fr, de, ru, zh, ja, pt-BR, ko, it) with 350+ translation keys
- **Events Service**: Real-time scan progress and health alert events
- **Encrypted Backups**: age-based encryption for backup archives
- **Performance**: Parallel duplicate detection, TTL caching, chunked file hashing
- **E2E Tests**: Playwright setup with dashboard, navigation, and theme tests
- **CI/CD**: GitHub Actions for Linux/Windows/macOS builds
- **Multi-platform**: cfg-guards for Windows/macOS compatibility
- **32 Unit Tests**: Path validation, response models, errors, security, cache service, kernel info

### Changed

- All 26 frontend views standardized with consistent layout pattern
- Theme service with smooth 300ms transition
- DataTable virtual scrolling for large lists
- System monitor uses requestAnimationFrame throttling
- Multiple services refactored for better organization

### Fixed

- Missing imports causing build errors
- Path validation for security
- All privileged file operations now logged to `/var/log/cleanux_privilege.log`
- Integration tests now have proper module visibility

### Deprecated

- Legacy non-lazy-loaded routes (replaced with lazy loading)

### Security

- Default-deny path allowlist
- Symlink traversal protection
- Input sanitization
- Privilege escalation auditing

## [0.2.1] - Previous Release

- Initial feature-complete release with 26 views, 31 routes, 32 services
