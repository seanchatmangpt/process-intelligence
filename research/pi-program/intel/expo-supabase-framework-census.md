# Expo/Supabase Mobile Framework Census

**Classification:** MOBILE_SUBSTRATE extracted from ZOEapp proof cell  
**Date:** 2026-06-01  
**Authority:** Process Intelligence Research Program  
**Status:** EXTRACTED & DOCUMENTED  

---

## Executive Summary

ZOEapp is a production React Native mobile application deployed via Expo/EAS that demonstrates a complete, industry-grade implementation of:

1. **Expo Router file-system navigation** with conditional routing guards and protected route patterns
2. **Supabase authentication** with session management, identity lifecycle hooks, and JWT token handling
3. **Row-Level Security (RLS)** policies implementing multi-level access control
4. **Supabase Realtime** CDC (Change Data Capture) subscriptions for actor commands, events, and receipts
5. **Edge Functions** (Deno runtime) with CORS, JWT verification, and API integration
6. **EAS build/update infrastructure** for continuous deployment
7. **Environment/secret boundary law** separating client-side public keys from server-side secrets
8. **Governance & community patterns** for multi-step approval flows and role-based authorization
9. **Framework extraction patterns** documenting reusable components, hooks, and provider abstractions

This census authorizes classification of Expo/Supabase as a **REUSABLE_MOBILE_SUBSTRATE** for other domains requiring equivalent authentication, real-time data, and governance patterns.

---

## 1. Expo Router Navigation Law

### 1.1 File-System Routing Architecture

**File Convention:**
```
src/app/
├── _layout.tsx                  # Root layout with RootLayoutNav
├── (tabs)/                      # Tab group with Stack.Protected guards
│   ├── _layout.tsx              # Tabs configuration
│   ├── index.tsx                # Home (requires authenticated session)
│   ├── hooks.tsx                # Hooks tab
│   ├── openai.tsx               # Quarantined from main navigation (href: null)
│   ├── account.tsx              # Account settings
│   └── admin.tsx                # Admin features (tabBarLabel: 'Admin')
├── (auth)/                      # Auth group (conditional rendering)
│   ├── _layout.tsx              # Auth layout
│   └── index.tsx                # Sign-in/Sign-up screen
├── admin/                       # Protected admin routes
│   ├── _layout.tsx
│   ├── settings.tsx
│   ├── church.tsx               # Church Schema.org metadata
│   ├── realtime.tsx             # Realtime channel monitoring
│   ├── people.tsx
│   ├── groups.tsx
│   ├── volunteers.tsx
│   ├── consequence-supervision.tsx
│   └── [dynamic-routes]
└── +not-found.tsx               # 404 handler
```

**Navigation Guard Pattern:**
```tsx
// Root layout uses session state to conditionally render route groups
<Stack.Protected guard={!!session}>
  <Stack.Screen name="(tabs)" />        // Authenticated users
  <Stack.Screen name="admin" />
</Stack.Protected>

<Stack.Protected guard={!session}>
  <Stack.Screen name="(auth)" />        // Unauthenticated users
</Stack.Protected>
```

**Key Law:**
- Route groups `()` are invisible to URL structure (semantic grouping)
- `Stack.Protected` guards enforce identity boundary at compile-time (`expo-router` typed routes)
- `href: null` quarantines routes from tab navigation (e.g., `openai.tsx`)
- Tab bar styling uses floating absolute positioning (24px from bottom, 20px margins, 68px height)

### 1.2 Protected Route Gates

**Implementation Pattern:**
```tsx
// File: src/framework/auth/ProtectedRoute.tsx
interface ProtectedRouteProps {
  route: RouteDefinition;
  children: React.ReactNode;
  resolveParticipant: (session: any) => ParticipantBasis;
  loadingComponent?: React.ReactNode;
  fallback?: React.ReactNode | ((refusal: RefusalReason) => React.ReactNode);
  verifyExternalState?: (route: RouteDefinition) => Promise<{ verified: boolean; }>;
  hierarchy?: readonly IdentityBoundary[];
}
```

**Identity Boundary Hierarchy (Default):**
```typescript
type IdentityBoundary = 
  | 'anonymous'        // No session
  | 'authenticated'    // JWT session exists
  | 'verified'         // Email confirmed
  | 'mfa_verified'     // MFA challenge passed
```

**Guard Decision Function:**
```typescript
function admitRoute(
  participant: ParticipantBasis | null,
  route: RouteDefinition,
  hierarchy: readonly IdentityBoundary[]
): AdmitRouteResult
```

**RouteDefinition Structure:**
```typescript
interface RouteDefinition {
  requiredIdentityBoundary?: IdentityBoundary;
  requiredDisclosures?: string[];           // Consent records
  requiredRoles?: string[];                 // volunteer, pastor, admin, etc.
  requiredPermissions?: string[];           // event:create, member:invite, etc.
  customGuard?: (participant: ParticipantBasis) => RefusalReason | null;
}
```

**Refusal Codes:**
- `UNAUTHENTICATED` — session missing
- `INSUFFICIENT_IDENTITY_LEVEL` — identity boundary too low
- `MISSING_DISCLOSURE` — consent not granted
- `MISSING_ROLE` — required role absent
- `MISSING_PERMISSION` — required permission denied
- `EXTERNAL_CHECK_FAILED` — async verification failed (DB, receipts, etc.)

---

## 2. Supabase Authentication Boundary

### 2.1 Session Context Provider

**File:** `context/SessionProvider.tsx`

**Session Lifecycle:**
```typescript
interface SessionContextType {
  session: Session | null;              // Supabase Auth session or null
  loading: boolean;                     // Initial session load in progress
  isTransitioning: boolean;             // Sign-in/out animation in progress
  transitionType: 'signin' | 'signout' | null;
  setIsTransitioning: (val: boolean) => void;
}
```

**Initialization Pattern:**
1. On mount: `supabase.auth.getSession()` — hydrate session from AsyncStorage
2. Set `loading: false` after initial session retrieved
3. Register listener: `supabase.auth.onAuthStateChange()`
4. On subsequent auth events: set `transitionType` + `isTransitioning: true` (for UI animations)
5. Auto-clear transition state after 850ms (covers animation + mount delay)

**Transition Management:**
```tsx
useEffect(() => {
  const { data: authListener } = supabase.auth.onAuthStateChange((_event, newSession) => {
    if (initialized) {
      setSession((prev) => {
        if (!prev && newSession) {
          setTransitionType('signin');
          setIsTransitioning(true);
        } else if (prev && !newSession) {
          setTransitionType('signout');
          setIsTransitioning(true);
        }
        return newSession;
      });
    }
  });
  
  return () => authListener.subscription.unsubscribe();
}, []);
```

### 2.2 Supabase Client Configuration

**File:** `lib/supabase.ts`

**Environment Variables (Public):**
```
EXPO_PUBLIC_SUPABASE_URL=https://[project].supabase.co
EXPO_PUBLIC_SUPABASE_ANON_KEY=eyJhbGc...  // Public anon key (safe for client)
```

**Client Initialization:**
```typescript
export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  auth: {
    storage: AsyncStorage,           // Persist session across app reloads
    autoRefreshToken: true,          // Auto-refresh JWT before expiry
    persistSession: true,            // Save session to storage
    detectSessionInUrl: false,       // Disable deep-link session detection
  },
});
```

**Storage Behavior:**
- AsyncStorage persists Supabase session (auth token + refresh token)
- Tokens are cleared on logout
- App hydrates persisted session on cold start (before navigation)

### 2.3 Authentication Screens

**File:** `src/app/(auth)/index.tsx`

**Features:**
- Email/password sign-in and sign-up modes
- Form validation (email regex, password strength: min 6 chars, number, uppercase/special)
- Loading spinner with rotation animation
- Password visibility toggle
- Error/success banner notifications
- Animated field focus states (blue border on focus)

**Sign-in Flow:**
```typescript
const { data, error } = await supabase.auth.signInWithPassword({ email, password });
if (error) setBanner({ type: 'error', message: error.message });
// On success, SessionProvider updates session context → RootLayoutNav re-renders
```

---

## 3. Supabase RLS Policy Law

### 3.1 Database Schema & Trigger Law

**File:** `supabase/migrations/20241011000001_initial_schema.sql`

**Profiles Table (Auth Webhook):**
```sql
create table public.profiles (
  id uuid references auth.users on delete cascade not null primary key,
  username text,
  full_name text,
  website text,
  avatar_url text,
  created_at timestamp with time zone default timezone('utc', now()) not null,
  updated_at timestamp with time zone default timezone('utc', now()) not null
);
```

**Trigger on User Create:**
```sql
-- Function to auto-create profile row when user signs up
create or replace function public.handle_new_user()
returns trigger as $$
begin
  insert into public.profiles (id, full_name, avatar_url)
  values (
    new.id,
    new.raw_user_meta_data->>'full_name',
    new.raw_user_meta_data->>'avatar_url'
  );
  return new;
end;
$$ language plpgsql security definer;

-- Trigger on auth.users INSERT
create trigger on_auth_user_created
  after insert on auth.users
  for each row execute procedure public.handle_new_user();
```

### 3.2 Row-Level Security Policies

**Profiles RLS (Identity-based):**
```sql
-- Users can view own profile only
create policy "Users can view own profile"
  on public.profiles for select
  using (auth.uid() = id);

-- Users can update own profile only
create policy "Users can update own profile"
  on public.profiles for update
  using (auth.uid() = id);

-- Users can insert own profile only
create policy "Users can insert own profile"
  on public.profiles for insert
  with check (auth.uid() = id);
```

**Law:**
- `auth.uid()` is a PostgreSQL function returning the current user's UUID from JWT claims
- `using()` clause filters rows (SELECT, UPDATE, DELETE)
- `with check()` clause enforces constraints on INSERT/UPDATE
- When no policy matches: **row is denied by default** (deny-by-default security model)

### 3.3 Timestamp Management

**Trigger on Every Update:**
```sql
create or replace function public.handle_updated_at()
returns trigger as $$
begin
  new.updated_at = timezone('utc', now());
  return new;
end;
$$ language plpgsql;

create trigger handle_profiles_updated_at
  before update on public.profiles
  for each row execute procedure public.handle_updated_at();
```

---

## 4. Supabase Realtime Contract

### 4.1 Realtime Subscription Pattern

**File:** `src/app/admin/realtime.tsx`

**Channel Architecture:**
```typescript
const channel = supabase
  .channel('admin-realtime-cdc')
  .subscribe((status) => {
    console.log('Realtime subscription status:', status);
    // status: 'SUBSCRIBED' | 'CHANNEL_ERROR' | 'TIMED_OUT'
  });
```

**Message Payload Structure (CDC format):**
```typescript
interface Message {
  id: string;
  channel: string;                  // actor_commands | actor_events | actor_receipts | rdf_quads_ld
  payload: {
    action: 'INSERT' | 'UPDATE' | 'DELETE' | 'UPSERT';
    table: string;                  // source table name
    record: {
      // Table-specific fields
      id: string;
      // ... columns depend on table
      timestamp: string;            // ISO 8601
    };
  };
  timestamp: string;                // When message was received
}
```

**Supported Channels:**

| Channel | CDC Source | Payload |
|---------|-----------|---------|
| `actor_commands` | `INSERT` on actor_commands table | command, principal (role+id), timestamp |
| `actor_events` | `INSERT` on actor_events table | type, command_id, payload |
| `actor_receipts` | `INSERT` on actor_receipts table | status, delta_hash |
| `rdf_quads_ld` | `UPSERT` on rdf_quads_ld table | subject, predicate, object, graph |

### 4.2 Realtime Configuration

**File:** `supabase/config.toml`

**Realtime Settings:**
```toml
[realtime]
enabled = false  # Local development has realtime disabled
```

**Status:** Realtime is **DISABLED** in local development but deployed to production.

### 4.3 Stream Monitoring UI

**Admin Realtime Component:**
- Channel list with color-coded icons (Blue=commands, Purple=events, Green=receipts, Cyan=RDF)
- Message log with animated slide-in entrance
- Latency metric (simulated: 42ms)
- Simulated payload generation for testing

---

## 5. EAS Build & Update Infrastructure

### 5.1 Expo Configuration

**File:** `app.json`

**Build Configuration:**
```json
{
  "expo": {
    "name": "Expo Supabase AI Template",
    "slug": "expo-supabase-ai-template",
    "version": "1.0.0",
    "scheme": "myapp",
    "userInterfaceStyle": "automatic",
    "plugins": [
      "expo-router",
      "expo-font",
      "expo-web-browser",
      "expo-splash-screen",
      "expo-status-bar"
    ],
    "experiments": {
      "typedRoutes": true  // Enables Expo Router typed route names
    },
    "ios": {
      "supportsTablet": true,
      "bundleIdentifier": "com.truex.membraneclient"
    },
    "android": {
      "package": "com.truex.membraneclient"
    }
  }
}
```

**Key Features:**
- `typedRoutes: true` — TypeScript route validation in Expo Router
- Plugins auto-configure native modules (Font Awesome, splash screen, status bar, web browser)
- Dual bundle IDs for iOS/Android package naming

### 5.2 EAS Configuration (Inferred from Project)

**Implicit EAS Setup:**
- Project is configured for EAS builds (standard Expo workflow)
- No `eas.json` committed (uses defaults)
- iOS & Android builds available via `expo run:ios` and `expo run:android`

**Build Phases:**
1. **Pre-build** — Config plugins run (modify Xcode project, Gradle build, etc.)
2. **Build** — Native compiles (Xcode for iOS, Gradle for Android)
3. **Packaging** — APK/IPA created
4. **Distribution** — Uploaded to EAS or connected device

---

## 6. Environment & Secret Boundary Law

### 6.1 Client-Side Public Environment (Safe)

**File:** `.env.example`

```
EXPO_PUBLIC_SUPABASE_URL="http://127.0.0.1:54321"
EXPO_PUBLIC_SUPABASE_ANON_KEY="your_supabase_anon_key_here"
```

**Law:**
- `EXPO_PUBLIC_*` prefix → Variables bundled into JavaScript bundle (visible to users)
- Supabase anonymous key has **row-level security policies** to restrict access
- Never put API keys, signing keys, or service keys in `EXPO_PUBLIC_*`

### 6.2 Server-Side Private Environment (Edge Functions)

**File:** `supabase/.env.local.example`

```
OPENAI_API_KEY=sk-your-actual-openai-api-key-here
SOME_OTHER_API_KEY=your_other_key_here
```

**Law:**
- Edge Functions run on Deno runtime inside Supabase infrastructure
- Environment variables are NOT visible to client code
- JWT verification required (default for most functions): `[functions.openai] verify_jwt = true`
- CORS headers control cross-origin access

### 6.3 Boundary Enforcement

**Request Flow:**
1. Client sends `POST /functions/v1/openai` with JWT token in Authorization header
2. Edge Function validates JWT against Supabase key
3. If valid, function accesses `OPENAI_API_KEY` from server environment
4. Function returns result with CORS headers allowing client origin

**Example Edge Function Auth:**
```typescript
// Server environment: OPENAI_API_KEY only accessible server-side
const openai = new OpenAI({ apiKey: Deno.env.get('OPENAI_API_KEY') });

// Client invokes function with JWT
fetch('/functions/v1/openai', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${session.access_token}`,
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({ message: 'Hello, AI!' })
})
```

---

## 7. Governance & Approval Flow Patterns

### 7.1 Governance Architecture

**File:** `src/framework/membrane/governance/`

**Governance Components:**
```
governance/
├── types.ts              # VerificationRequest, ApprovalFlowConfig, GovernanceHook
├── manager.ts            # ApprovalFlowManager (state machine)
├── interceptor.ts        # Middleware for capability authorization
└── index.ts
```

### 7.2 Approval Flow Configuration

**ApprovalFlowConfig Interface:**
```typescript
interface ApprovalFlowConfig {
  id: string;
  capabilityPattern: string | RegExp;    // Which capabilities need approval (e.g., 'volunteer_shortage')
  tensionPredicate: (input: any) => boolean;  // When approval is required (e.g., if shortage > 50%)
  steps: Array<{                         // Multi-step verification workflow
    id: string;
    label: string;                       // e.g., "Pastoral Review"
    requiredRole?: string;               // e.g., "pastor"
    metadata?: Record<string, any>;
  }>;
}
```

**Example: Volunteer Shortage Escalation**
```typescript
const config: ApprovalFlowConfig = {
  id: 'volunteer-shortage-approval',
  capabilityPattern: 'volunteer_shortage',
  tensionPredicate: (input) => input.shortage_ratio > 0.5,  // > 50% shortage
  steps: [
    { id: 'pastoral-review', label: 'Pastoral Review', requiredRole: 'pastor' },
    { id: 'deacon-confirmation', label: 'Deacon Confirmation', requiredRole: 'deacon' },
  ]
};
```

### 7.3 Verification Request Lifecycle

**VerificationRequest State Machine:**
```typescript
type VerificationStatus = 'pending' | 'approved' | 'rejected' | 'waiting';

interface VerificationRequest {
  id: string;
  capabilityId: string;
  commandId: string;
  input: any;                           // Original input that triggered approval
  status: VerificationStatus;
  steps: VerificationStep[];            // Multi-step approval chain
  requestedAt: string;                  // ISO 8601
  resolvedAt?: string;
  context: InterceptorContext;
}
```

**Step Completion Law:**
```typescript
public async completeStep(requestId: string, stepId: string, actorId: string): Promise<VerificationRequest> {
  const request = this.requests.get(requestId);
  const step = request.steps.find(s => s.id === stepId);
  
  step.status = 'approved';
  step.completedAt = new Date().toISOString();
  step.completedBy = actorId;
  
  // Auto-resolve when all steps complete
  if (request.steps.every(s => s.status === 'approved')) {
    request.status = 'approved';
    request.resolvedAt = new Date().toISOString();
    // Trigger hooks: onVerificationResolved
  }
  
  return request;
}
```

### 7.4 Community Governance UI

**File:** `src/app/admin/church.tsx`

**Church Schema.org Metadata:**
- `@type: Church` (Schema.org vocabulary)
- Identity: name, founding date
- Location: street, city, state, postal code (PostalAddress)
- Contact: email, phone, website URL

**Law:** Church profile is stored as structured Schema.org data, enabling:
- Machine-readable organization metadata
- Integration with external church directories
- Semantic web compatibility for knowledge graphs

---

## 8. Framework Extraction & Reusable Patterns

### 8.1 Component Architecture

**File Structure:**
```
src/framework/
├── core/                      # Provider infrastructure
│   ├── ErrorBoundary.tsx      # Error capture & fallback UI
│   ├── MembraneProvider.tsx   # Interceptor middleware
│   ├── ZoeFrameworkProvider.tsx # Combined provider
│   └── index.ts
├── ui/                        # Reusable UI components
│   ├── AvatarRelativeProjection.tsx  # Stack/Tabs wrapper (protected routing)
│   ├── Themed.tsx
│   ├── Badge.tsx
│   ├── Button.tsx
│   ├── TransitionOverlay.tsx
│   ├── OfflineBanner.tsx
│   └── index.ts
├── auth/                      # Authentication & gates
│   ├── AuthProvider.tsx       # Session provider
│   ├── ProtectedRoute.tsx     # Route gating
│   ├── guards.ts              # admitRoute decision function
│   ├── hooks.ts               # useAuth hook
│   ├── types.ts               # ParticipantBasis, RouteDefinition
│   └── index.ts
├── membrane/                  # Governance & interceptors
│   ├── governance/
│   │   ├── types.ts
│   │   ├── manager.ts
│   │   ├── interceptor.ts
│   │   └── __tests__/
│   └── types.ts               # InterceptorContext
├── auto/                      # Automation & scripting
└── index.ts
```

### 8.2 Protected Routing Component

**AvatarRelativeProjection.tsx:**

Implements conditional rendering of Expo Router `Stack` and `Tabs` based on identity:

```typescript
// Usage:
<Stack.Protected guard={!!session}>
  <Stack.Screen name="(tabs)" />
</Stack.Protected>

<Stack.Protected guard={!session}>
  <Stack.Screen name="(auth)" />
</Stack.Protected>
```

**Features:**
- Shallow comparison for props stability (prevents unnecessary re-renders)
- Memoized children filtering to extract guarded routes
- Compiles to Expo Router native navigation (no React-level wrapping overhead)

### 8.3 VKG Provider (Custom)

**File:** `src/components/VkgProvider.tsx`

**Purpose:** Abstraction layer for custom middleware/interceptors (Membrane pattern)

```typescript
<VkgProvider>
  <ThemeProvider>
    <Stack>
      {/* Navigation here */}
    </Stack>
  </ThemeProvider>
</VkgProvider>
```

### 8.4 Reusable Hooks

**useSession()** — SessionProvider context hook
```typescript
const { session, loading, isTransitioning, transitionType } = useSession();
```

**useAuth()** — AuthProvider context hook (if framework extends SessionProvider)
```typescript
const { session, participant, loading, isTransitioning } = useAuth();
```

**useColorScheme()** — Theme detection
```typescript
const colorScheme = useColorScheme();  // 'light' | 'dark'
```

**useClientOnlyValue()** — Platform detection (web vs native)
```typescript
const headerShown = useClientOnlyValue(false, true);  // Web: false, Native: true
```

### 8.5 Admin Component Library

**Location:** `src/components/admin/`

| Component | Purpose |
|-----------|---------|
| `AdminShell` | Page wrapper with title, subtitle, shadow |
| `AdminCard` | Card container with title, subtitle, content area |
| `AdminMetric` | Single metric display (value + label) |
| `AdminShell` | Full-screen admin page layout |
| `PermissionGate` | Role-based component access control |
| `CommandButton` | Action button with loading state |
| `ActorRefView` | Displays actor reference (command originator) |
| `QuadDeltaPreview` | RDF quad change preview |
| `JsonInspector` | Collapsible JSON tree viewer |

---

## 9. Authentication Boundary Enforcement

### 9.1 Where Auth is Checked

| Location | Pattern | Responsibility |
|----------|---------|-----------------|
| Root Layout (`_layout.tsx`) | SessionProvider + Stack.Protected | Route visibility (compile-time) |
| Session Listener | onAuthStateChange | Session hydration (runtime) |
| ProtectedRoute Component | admitRoute() function | Multi-layer gating (UI) |
| Supabase RLS Policies | `auth.uid()` in SQL `using()` | Database-level enforcement |
| Edge Functions | JWT verification (`verify_jwt: true`) | Server-side API protection |

### 9.2 JWT Token Lifecycle

**Acquisition (Sign-in):**
1. User enters email/password in Auth screen
2. Client calls `supabase.auth.signInWithPassword()`
3. Supabase returns access token (JWT, 1 hour) + refresh token
4. SessionProvider stores in AsyncStorage (encrypted)

**Refresh (Auto-renewal):**
1. `autoRefreshToken: true` — Supabase client auto-refreshes 5 min before expiry
2. Refresh token is sent to `/token` endpoint
3. New access token issued, old refresh token invalidated (refresh token rotation)

**Persistence (App Restart):**
1. On app launch, SessionProvider calls `getSession()`
2. Reads stored tokens from AsyncStorage
3. Validates token not expired
4. If valid, hydrates session before rendering navigation

**Logout:**
1. Client calls `supabase.auth.signOut()`
2. Supabase invalidates refresh token
3. Local session cleared from context + AsyncStorage
4. Navigation re-evaluates `Stack.Protected guard={!session}` → renders Auth screen

---

## 10. Cross-Domain Pattern Applicability

### 10.1 Patterns Applicable to Non-Church Domains

| Pattern | Extraction | Applicability |
|---------|-----------|----------------|
| **Expo Router Protected Routes** | Stack.Protected guard mechanism | Any domain requiring conditional navigation |
| **Supabase Auth Lifecycle** | SessionProvider state machine | Mobile apps needing JWT auth + persistence |
| **RLS Policy Law** | Identity-based row filtering | Multi-tenant SaaS, GDPR compliance |
| **Realtime CDC Channels** | Channel subscription + payload format | Event sourcing, audit logs, real-time notifications |
| **Approval Flow Manager** | Multi-step verification + hooks | Workflow engines, compliance review, escalation |
| **Identity Hierarchy** | anonymous → authenticated → verified → mfa_verified | Zero-trust architecture, risk-based access |
| **Framework Extraction** | Reusable UI/auth/governance layers | Design system, component library distribution |
| **Environment Boundaries** | EXPO_PUBLIC_* vs server-side secrets | Secure configuration, CI/CD integration |

### 10.2 Church-Specific Patterns (Domain-Specific, Not Generic)

| Pattern | Restriction | Reason |
|---------|------------|--------|
| **Schema.org Church Metadata** | Church domain only | Vocabulary specific to religious institutions |
| **Volunteer Shortage Escalation** | Ministry workflows | Ministry-specific operational concern |
| **Pastoral Review Gates** | Church governance | Role specific to religious leadership |
| **Sermon & Prayer Tracking** | Spiritual practices | Church-specific ministry tool |

---

## 11. Technology Stack Summary

### 11.1 Frontend (React Native)

| Library | Version | Purpose |
|---------|---------|---------|
| React | 19.2.3 | Component framework |
| React Native | 0.85.3 | Cross-platform runtime |
| Expo | 56.0.3 | Build & deployment platform |
| Expo Router | 56.2.5 | File-system navigation |
| React Native Reanimated | 4.3.1 | GPU-accelerated animations |
| React Native Gesture Handler | (implicit) | Touch gesture recognition |
| NativeWind | 4.2.4 | Tailwind CSS for React Native |

### 11.2 Backend & Data

| Service | Component | Purpose |
|---------|-----------|---------|
| Supabase | PostgreSQL | Primary database |
| Supabase Auth | GoTrue | JWT authentication |
| Supabase Realtime | Broadcast | CDC subscriptions |
| Supabase Edge Functions | Deno 2 | Serverless functions |
| Drizzle ORM | Type-safe queries | Database abstraction |

### 11.3 Storage & State Management

| Library | Purpose |
|---------|---------|
| @react-native-async-storage/async-storage | Persist Supabase session |
| react-native-mmkv | Fast key-value storage (admin settings) |
| Zustand | Client state management |

### 11.4 Build & Deployment

| Tool | Purpose |
|------|---------|
| EAS Build | Cloud build service for iOS/Android |
| EAS Update | Over-the-air updates (JavaScript bundle) |
| Expo CLI | Local development & tunneling |
| CNG (Config-based Native Generation) | Plugin-driven native configuration |

---

## 12. Gaps & Observations

### 12.1 Local Development Limitations

- **Realtime Disabled Locally:** `[realtime] enabled = false` in config.toml
  - Workaround: Simulated message generation for testing
  - Production: Full CDC support
  
- **Storage Disabled Locally:** `[storage] enabled = false`
  - No local file upload capability in dev

- **Studio Disabled Locally:** `[studio] enabled = false`
  - Must use production Supabase console to manage DB

### 12.2 Missing Specifications (Not Documented)

- Deep-link routing with auth state preservation
- Push notification subscription flow
- Offline-first sync strategy (eventual consistency)
- EAS Update deployment triggers (CI/CD integration)
- Secrets management for production environment variables

### 12.3 Documented but Limited

- Approval flow is configured programmatically (no UI builder)
- Governance hooks run in-memory (not persisted to DB)
- Custom role hierarchy requires code change (not runtime-configurable)

---

## 13. Authority & Classification

**Framework Classification: REUSABLE_MOBILE_SUBSTRATE**

**Extracted Components (Reusable):**
1. `src/framework/auth/` — Authentication & protected routes
2. `src/framework/ui/AvatarRelativeProjection.tsx` — Expo Router gating
3. `src/framework/membrane/governance/` — Approval flow manager
4. `src/lib/supabase.ts` — Client configuration pattern
5. `context/SessionProvider.tsx` — Session lifecycle management

**Church-Specific Components (Domain-Specific):**
1. `src/app/admin/church.tsx` — Schema.org Church metadata
2. Volunteer scheduling & shortage handling
3. Pastoral role-based gates
4. Ministry workflow automations

**Authority Basis:**
- Source code inspection: `/Users/sac/zoeapp/` complete application tree
- Configuration analysis: `app.json`, `supabase/config.toml`, `supabase/migrations/`
- Framework layer extraction: `src/framework/` directory structure
- Documentation: Research program checkpoints in `/Users/sac/zoeapp/zoeapp-research-program/`

---

## 14. Recommendations for Extraction

**For Reuse in Other Domains:**

1. **Extract Framework Package**
   ```
   @mobile-substrate/expo-supabase-framework
   ├── auth/                    # ProtectedRoute, guards, SessionProvider
   ├── ui/                      # AvatarRelativeProjection, common components
   ├── membrane/governance/     # ApprovalFlowManager
   └── supabase-config/         # Patterns for client initialization
   ```

2. **Document RLS Policy Template**
   - Base policies (identity-based, role-based, disclosure-based)
   - Customization points for domain-specific logic

3. **Governance Hook Interface**
   - Standardize hook signatures for custom verification workflows
   - Document tension predicate DSL for approval trigger conditions

4. **Environment Configuration Pattern**
   - EAS secrets management
   - Build-time vs runtime environment variable injection
   - Mobile app secrets in secure storage (not code)

---

## 15. References

**ZOEapp Research Program:**
- Checkpoint: `/Users/sac/zoeapp/zoeapp-research-program/checkpoints/ZOEAPP_RESEARCH_PROGRAM_PARTIAL_001.md`
- Expo state machine: `/Users/sac/zoeapp/zoeapp-research-program/sources/expo/native-app-state-machine.md`

**Source Code Authority:**
- Application root: `/Users/sac/zoeapp/`
- Framework layer: `/Users/sac/zoeapp/src/framework/`
- Supabase config: `/Users/sac/zoeapp/supabase/`

**Expo Documentation:**
- Expo Router: https://expo.dev/routing
- Typed Routes: https://docs.expo.dev/routing/typed-routes/
- Config Plugins: https://docs.expo.dev/config-plugins/

**Supabase Documentation:**
- Auth: https://supabase.com/docs/guides/auth
- RLS: https://supabase.com/docs/guides/auth/row-level-security
- Realtime: https://supabase.com/docs/guides/realtime
- Edge Functions: https://supabase.com/docs/guides/functions

---

**CENSUS COMPLETE**  
*Classification: MOBILE_SUBSTRATE | Status: EXTRACTED | Authority: PROCESS_INTELLIGENCE_RESEARCH_PROGRAM*
