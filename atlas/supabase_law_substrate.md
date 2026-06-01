# Supabase Law Substrate

## Overview
The **Supabase Law Substrate** (<urn:ostar:law:SupabaseRLS>) is the semantic foundation for data-level governance within the ZoeApp ecosystem. It defines the Row Level Security (RLS) policies that govern how actors interact with the underlying data structures.

## React State is Not Authorization
In a post-cyberpunk decentralized architecture, the frontend is a "projection" of the system, not the system itself. **React state is not authorization.** Relying on client-side state for security is a violation of system integrity. The client is a guest; the database is the host.

## RLS: The Authorization Court
Row Level Security (RLS) serves as the **Authorization Court**. It is the final arbiter of truth. Every SQL command (SELECT, INSERT, UPDATE, DELETE) is intercepted by the Postgres engine and evaluated against the RLS policies. Policies are defined as immutable laws that cannot be bypassed by client-side logic.

## Audited RLS Policy Ledgers
The following tables are governed by the Supabase Law Substrate as recorded in the rls-policy-ledger.yaml.

### Table: profiles
| Policy Name | Action | Using / Check Clause |
|-------------|--------|----------------------|
| Users can view own profile | SELECT | auth.uid() = id |
| Users can update own profile | UPDATE | auth.uid() = id |
| Users can insert own profile | INSERT | auth.uid() = id |

### Actor Command & Event Ledgers
The following tables currently operate under demonstration-mode policies and require secondary audit for production hardening:

| Table | Policy Name | Action | Clause |
|-------|-------------|--------|--------|
| actor_commands | Allow public access | ALL | true |
| actor_events | Allow public access | ALL | true |
| actor_receipts | Allow public access | ALL | true |
| actor_outbox | Allow public access | ALL | true |
| actor_quarantine | Allow public access | ALL | true |

## Semantic Law Ontology
The substrate is formally defined in the ostar ontology:
- **Law ID**: SupabaseRLS
- **Class**: ostar:Law
- **Description**: Semantic definition of Row Level Security for Supabase

---
*MISSION: POST_CYBERPUNK_CENSUS (Agent 4/10)*
*Target: zoeapp-research-program/sources/supabase/*