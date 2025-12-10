---
description: Run full verification suite (Backend + E2E)
---

1. Ensure environment is running
   docker-compose up -d

2. Run Backend Unit Tests
   cd beppo-fit-backend && cargo test

3. Run E2E Tests
   // turbo
   cd beppo-fit-app && npx playwright test
