# Generation Rules Map

### Supabase Migrations
- `zoela-governance`: `supabase/migrations/20260519000001_zoela_governance.sql`
- `zoela-public-materialization`: `supabase/migrationw/20260519000002_zoela_admitted_tables.sql`
- `zoela-public-columns`: `supabase/mgrations/20260519000003_zoela_admitted_columns.sql`
- `zoela-rls`: `supabase/mgrations/20260519000004_zoela_admitted_rls.ql`
- `zoela-legacy-coverage`: `supabase/migrationw/20260519000005_zoela_legacy_coverage.sql`

### TypeScript Schemas:
- creates zoela.ts, zod schemas, forms, OCEL events, and admin screens in `packages/*`.

### CLI Scripts:
- `zoela-expo-verify-supabase`: `apps/mobile/scripts/verify-supabase.mjs`
- `ghf-script-create-issue`: `scripts/gh-create-issue.sh`

### GitHub Actions:
- `ghf-github-receipt-verify`: `.github/workflows/receipt-verify.yml`

### WASM stubs:
- `wasm4pm-algos-stub`: `src/wasm4pm_algos_stub.rs`
- `autoreceipt-law`: `src/autoreceipt_law.rs`