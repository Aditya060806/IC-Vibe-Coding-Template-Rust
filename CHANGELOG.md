# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add per-principal counter system with stable storage for persistent user-specific counters
- Add NFID Wallet authentication using Identity Kit for secure user login
- Add sign-in/sign-out authentication state management with counter access control
- Add navigation header with logo, title, and links to main sections
- Add set_count update method to allow setting the counter to a specific value
- Add frontend development server scripts (`npm run start`)
- Add LLM canister implementation
- Add IdentityKit integration for authenticated counter operations with delegation support
- Add Internet Identity local deployment for development with IdentityKit

### Changed

- Update dependencies to latest versions

## [0.1.0] - 2025-04-24

### Added

- Basic canister structure with Rust
- Counter functionality with increment and get_count methods
- Greeting functionality
- PocketIC testing infrastructure
- Vitest test runner configuration
- GitHub CI workflow for automated end-to-end tests for all methods
- Project documentation
- Add custom instructions for github copilot
