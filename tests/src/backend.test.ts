import { describe, beforeEach, afterEach, it, expect, inject } from "vitest";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import { PocketIc, type Actor } from "@dfinity/pic";
import { Principal } from "@dfinity/principal";

// Import generated types for your canister
import {
  type _SERVICE,
  idlFactory,
} from "../../src/declarations/backend/backend.did.js";

// Define the path to your canister's WASM file
export const WASM_PATH = resolve(
  dirname(fileURLToPath(import.meta.url)),
  "..",
  "..",
  "target",
  "wasm32-unknown-unknown",
  "release",
  "backend.wasm",
);

// The `describe` function is used to group tests together
describe("Vibe Coding Template Backend", () => {
  // Define variables to hold our PocketIC instance, canister ID,
  // and an actor to interact with our canister.
  let pic: PocketIc;
  // @ts-ignore - This variable is used in the setup / framework
  let canisterId: Principal;
  let actor: Actor<_SERVICE>;

  // The `beforeEach` hook runs before each test.
  beforeEach(async () => {
    // create a new PocketIC instance
    pic = await PocketIc.create(inject("PIC_URL"));

    // Setup the canister and actor
    const fixture = await pic.setupCanister<_SERVICE>({
      idlFactory,
      wasm: WASM_PATH,
    });

    // Save the actor and canister ID for use in tests
    actor = fixture.actor;
    canisterId = fixture.canisterId;
  });

  // The `afterEach` hook runs after each test.
  afterEach(async () => {
    // tear down the PocketIC instance
    await pic.tearDown();
  });

  // The `it` function is used to define individual tests
  it("should greet with the provided name", async () => {
    const response = await actor.greet("World");
    expect(response).toEqual("Hello, World!");
  });

  it("should increment counter per principal", async () => {
    // Setup: Get initial count for the current principal (should be 0)
    const initialCount = await actor.get_count();
    expect(initialCount).toEqual(BigInt(0));

    // Execute: Increment the counter
    const newCount = await actor.increment();

    // Assert: Counter should be incremented to 1
    expect(newCount).toEqual(BigInt(1));

    // Verify get_count returns the same value
    const currentCount = await actor.get_count();
    expect(currentCount).toEqual(BigInt(1));
  });

  it("should maintain separate counters for different principals", async () => {
    // Setup: Create two different principals
    const principal1 = Principal.fromUint8Array(new Uint8Array([1, 2, 3, 4]));
    const principal2 = Principal.fromUint8Array(new Uint8Array([5, 6, 7, 8]));

    // Create separate actors for each principal
    const actor1 = pic.createActor<_SERVICE>(idlFactory, canisterId);
    actor1.setPrincipal(principal1);

    const actor2 = pic.createActor<_SERVICE>(idlFactory, canisterId);
    actor2.setPrincipal(principal2);

    // Execute: Both principals start with 0 count
    const initialCount1 = await actor1.get_count();
    const initialCount2 = await actor2.get_count();
    expect(initialCount1).toEqual(BigInt(0));
    expect(initialCount2).toEqual(BigInt(0));

    // Execute: Principal1 increments twice
    await actor1.increment(); // Should be 1
    const count1AfterTwo = await actor1.increment(); // Should be 2
    expect(count1AfterTwo).toEqual(BigInt(2));

    // Execute: Principal2 increments once
    const count2AfterOne = await actor2.increment(); // Should be 1
    expect(count2AfterOne).toEqual(BigInt(1));

    // Assert: Verify each principal maintains their own counter
    const finalCount1 = await actor1.get_count();
    const finalCount2 = await actor2.get_count();

    expect(finalCount1).toEqual(BigInt(2));
    expect(finalCount2).toEqual(BigInt(1));

    // Execute: Principal1 increments again
    const count1AfterThree = await actor1.increment(); // Should be 3
    expect(count1AfterThree).toEqual(BigInt(3));

    // Assert: Principal2's counter should remain unchanged
    const unchangedCount2 = await actor2.get_count();
    expect(unchangedCount2).toEqual(BigInt(1));
  });

  it("should return 0 for new principal with no previous increments", async () => {
    // Setup & Execute: Get count for a principal that hasn't incremented
    const count = await actor.get_count();

    // Assert: Should return 0 for new principal
    expect(count).toEqual(BigInt(0));
  });

  it("should set counter to specified value for calling principal", async () => {
    // Setup: Start with any counter value
    await actor.increment(); // Set to 1

    // Execute: Set counter to a specific value
    const newValue = BigInt(42);
    const result = await actor.set_count(newValue);

    // Assert: Should return the set value
    expect(result).toEqual(newValue);

    // Verify get_count returns the same value
    const currentCount = await actor.get_count();
    expect(currentCount).toEqual(newValue);

    // Execute: Increment from the set value
    const incrementedCount = await actor.increment();

    // Assert: Should be the set value + 1
    expect(incrementedCount).toEqual(newValue + BigInt(1));
  });

  it("should set different counter values for different principals", async () => {
    // Setup: Create two different principals
    const alice = Principal.fromUint8Array(new Uint8Array([10, 20, 30]));
    const bob = Principal.fromUint8Array(new Uint8Array([40, 50, 60]));

    const aliceActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    aliceActor.setPrincipal(alice);

    const bobActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    bobActor.setPrincipal(bob);

    // Execute: Set different values for each principal
    const aliceValue = BigInt(100);
    const bobValue = BigInt(200);

    const aliceResult = await aliceActor.set_count(aliceValue);
    const bobResult = await bobActor.set_count(bobValue);

    // Assert: Each principal should get their set value back
    expect(aliceResult).toEqual(aliceValue);
    expect(bobResult).toEqual(bobValue);

    // Verify: Each principal maintains their own value
    const aliceCount = await aliceActor.get_count();
    const bobCount = await bobActor.get_count();

    expect(aliceCount).toEqual(aliceValue);
    expect(bobCount).toEqual(bobValue);

    // Execute: Increment both and verify independence
    const aliceIncremented = await aliceActor.increment();
    const bobIncremented = await bobActor.increment();

    expect(aliceIncremented).toEqual(aliceValue + BigInt(1));
    expect(bobIncremented).toEqual(bobValue + BigInt(1));
  });

  it("should handle anonymous principal correctly", async () => {
    // Setup: Create an actor with anonymous identity
    const anonymousActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    anonymousActor.setPrincipal(Principal.anonymous());

    // Execute: Interact with anonymous principal
    const initialCount = await anonymousActor.get_count();
    expect(initialCount).toEqual(BigInt(0));

    // Execute: Set a value for anonymous principal
    const testValue = BigInt(500);
    const setResult = await anonymousActor.set_count(testValue);
    expect(setResult).toEqual(testValue);

    // Execute: Increment and verify
    const incrementedValue = await anonymousActor.increment();
    expect(incrementedValue).toEqual(testValue + BigInt(1));

    // Assert: Verify the default actor (also anonymous) sees the same values
    const defaultActorCount = await actor.get_count();
    expect(defaultActorCount).toEqual(testValue + BigInt(1));
  });

  it("should isolate counters between regular and anonymous principals", async () => {
    // Setup: Create a named principal actor
    const namedPrincipal = Principal.fromUint8Array(
      new Uint8Array([100, 101, 102]),
    );
    const namedActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    namedActor.setPrincipal(namedPrincipal);

    // Setup: Anonymous actor (default actor is already anonymous)
    const anonymousCount = await actor.get_count();
    expect(anonymousCount).toEqual(BigInt(0));

    // Execute: Set values for both principals
    await namedActor.set_count(BigInt(300));
    await actor.set_count(BigInt(400));

    // Assert: Each principal should maintain separate values
    const namedCount = await namedActor.get_count();
    const anonCount = await actor.get_count();

    expect(namedCount).toEqual(BigInt(300));
    expect(anonCount).toEqual(BigInt(400));

    // Execute: Increment both
    await namedActor.increment(); // 301
    await actor.increment(); // 401

    // Assert: Values remain isolated
    const finalNamedCount = await namedActor.get_count();
    const finalAnonCount = await actor.get_count();

    expect(finalNamedCount).toEqual(BigInt(301));
    expect(finalAnonCount).toEqual(BigInt(401));
  });

  it("should handle multiple principals with complex operations", async () => {
    // Setup: Create multiple principals with meaningful IDs
    const alice = Principal.fromText("rdmx6-jaaaa-aaaaa-aaadq-cai");
    const bob = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
    const charlie = Principal.fromUint8Array(new Uint8Array([1, 2, 3, 4, 5]));

    const aliceActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    aliceActor.setPrincipal(alice);

    const bobActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    bobActor.setPrincipal(bob);

    const charlieActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    charlieActor.setPrincipal(charlie);

    // Execute: Each principal should start with 0
    const aliceInitial = await aliceActor.get_count();
    const bobInitial = await bobActor.get_count();
    const charlieInitial = await charlieActor.get_count();

    expect(aliceInitial).toEqual(BigInt(0));
    expect(bobInitial).toEqual(BigInt(0));
    expect(charlieInitial).toEqual(BigInt(0));

    // Execute: Complex operations - Alice sets high value, Bob increments, Charlie does both
    await aliceActor.set_count(BigInt(1000));

    await bobActor.increment(); // 1
    await bobActor.increment(); // 2
    await bobActor.increment(); // 3

    await charlieActor.set_count(BigInt(500));
    await charlieActor.increment(); // 501
    await charlieActor.increment(); // 502

    // Assert: Each principal should have different final counts
    const aliceFinal = await aliceActor.get_count();
    const bobFinal = await bobActor.get_count();
    const charlieFinal = await charlieActor.get_count();

    expect(aliceFinal).toEqual(BigInt(1000));
    expect(bobFinal).toEqual(BigInt(3));
    expect(charlieFinal).toEqual(BigInt(502));

    // Execute: Cross-verify by checking counts from different actors
    // Each actor should only see their own principal's count
    const aliceSeesAlice = await aliceActor.get_count();
    const bobSeesBob = await bobActor.get_count();
    const charlieSeesCharlie = await charlieActor.get_count();

    expect(aliceSeesAlice).toEqual(BigInt(1000));
    expect(bobSeesBob).toEqual(BigInt(3));
    expect(charlieSeesCharlie).toEqual(BigInt(502));
  });
});
