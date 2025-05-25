# Multi-Principal Testing with PocketIC

This document explains how to test canister functions with different principals using PocketIC in the Internet Computer ecosystem.

## Overview

PocketIC allows you to simulate multiple users (principals) interacting with your canister by creating separate actors with different principal identities. This is crucial for testing functions that rely on `ic_cdk::caller()` to maintain per-principal state.

## Key Concepts

### Principal Identification

In Internet Computer canisters, each caller has a unique `Principal` ID. Your canister can use `ic_cdk::caller()` to identify who is making the call and maintain separate state for different users.

### Actor Creation

PocketIC allows you to create multiple actors for the same canister, each with a different principal identity:

```typescript
// Create an actor with a specific principal
const actor = pic.createActor<_SERVICE>(idlFactory, canisterId);
actor.setPrincipal(principal);
```

## Implementation Examples

### 1. Creating Different Principals

```typescript
// Create principals from byte arrays
const alice = Principal.fromUint8Array(new Uint8Array([1, 2, 3, 4]));
const bob = Principal.fromUint8Array(new Uint8Array([5, 6, 7, 8]));

// Create principals from text (useful for known canister IDs)
const knownPrincipal = Principal.fromText("rdmx6-jaaaa-aaaaa-aaadq-cai");

// Use anonymous principal
const anonymous = Principal.anonymous();
```

### 2. Setting Up Actors for Different Principals

```typescript
const aliceActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
aliceActor.setPrincipal(alice);

const bobActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
bobActor.setPrincipal(bob);
```

### 3. Testing Principal Isolation

Our tests demonstrate several key scenarios:

#### Separate Counter State

Each principal maintains their own counter value:

```typescript
await aliceActor.set_count(BigInt(100));
await bobActor.set_count(BigInt(200));

const aliceCount = await aliceActor.get_count(); // Returns 100
const bobCount = await bobActor.get_count(); // Returns 200
```

#### Anonymous vs Named Principals

The default actor is anonymous, but you can create specifically anonymous actors:

```typescript
const anonymousActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
anonymousActor.setPrincipal(Principal.anonymous());
```

## Test Structure

### Setup Phase

```typescript
beforeEach(async () => {
  pic = await PocketIc.create(inject("PIC_URL"));
  const fixture = await pic.setupCanister<_SERVICE>({
    idlFactory,
    wasm: WASM_PATH,
  });
  actor = fixture.actor;
  canisterId = fixture.canisterId;
});
```

### Test Implementation

```typescript
it("should maintain separate counters for different principals", async () => {
  // Setup: Create different principals
  const principal1 = Principal.fromUint8Array(new Uint8Array([1, 2, 3, 4]));
  const principal2 = Principal.fromUint8Array(new Uint8Array([5, 6, 7, 8]));

  // Create separate actors
  const actor1 = pic.createActor<_SERVICE>(idlFactory, canisterId);
  actor1.setPrincipal(principal1);

  const actor2 = pic.createActor<_SERVICE>(idlFactory, canisterId);
  actor2.setPrincipal(principal2);

  // Execute & Assert: Test isolation
  await actor1.increment(); // Principal1: 1
  await actor2.increment(); // Principal2: 1

  const count1 = await actor1.get_count(); // Should be 1
  const count2 = await actor2.get_count(); // Should be 1

  expect(count1).toEqual(BigInt(1));
  expect(count2).toEqual(BigInt(1));
});
```

## Best Practices

1. **Use Descriptive Principal IDs**: Use meaningful byte arrays or known principal text formats for better test readability.

2. **Test Principal Isolation**: Always verify that actions by one principal don't affect another principal's state.

3. **Test Anonymous Principal**: Include tests for the anonymous principal since it's commonly used.

4. **Complex Scenarios**: Test scenarios where multiple principals perform different operations to ensure state isolation.

5. **Cleanup**: PocketIC automatically cleans up between tests with the `afterEach` hook.

## Running the Tests

```bash
# Check TypeScript compilation
npx tsc -p tests/tsconfig.json

# Run backend tests
npm run test:backend
```

## Common Pitfalls

1. **Reusing Actors**: Don't reuse actors between tests without resetting their principal.
2. **Default Principal**: Remember that the default actor is typically anonymous.
3. **Principal Format**: Ensure principals are properly formatted when creating from text or bytes.

This approach allows you to thoroughly test multi-user scenarios in your Internet Computer canisters, ensuring that user isolation and per-principal state management work correctly.
