---
title: Supabase and PGLite Integration
tags: [database, e2e, testing, supabase, pglite, playwright]
source_count: 1
---

# Supabase and PGLite Integration

## Concept Overview
CodeViz employs a split-database strategy to provide both an Enterprise-ready cloud database and a true-local E2E testing environment without relying on mocks.

- **Supabase**: Handles production workloads, multi-tenant RBAC, Teams, and OAuth.
- **PGLite**: A WASM-based in-memory PostgreSQL instance used strictly during Playwright E2E tests (`test:e2e`). 

## Key Mechanisms
When `E2E_TEST` is true, the application routes all database transactions to a locally instantiated PGLite memory space seeded by Playwright's test hooks. This guarantees tests execute against actual SQL engines without cloud dependencies.
