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

  it("should greet with the provided name", async () => {
    const response = await actor.greet("World");
    expect(response).toEqual("Hello, World!");
  });

  it("should maintain per-principal counters with increment and set operations", async () => {
    // Setup: Create two different principals
    const alice = Principal.fromUint8Array(new Uint8Array([1, 2, 3, 4]));
    const bob = Principal.fromUint8Array(new Uint8Array([5, 6, 7, 8]));

    const aliceActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    aliceActor.setPrincipal(alice);

    const bobActor = pic.createActor<_SERVICE>(idlFactory, canisterId);
    bobActor.setPrincipal(bob);

    // Execute: Both principals start with 0 count
    expect(await aliceActor.get_count()).toEqual(BigInt(0));
    expect(await bobActor.get_count()).toEqual(BigInt(0));

    // Execute: Alice increments twice, Bob sets then increments
    await aliceActor.increment();
    const aliceCount = await aliceActor.increment();
    expect(aliceCount).toEqual(BigInt(2));

    await bobActor.set_count(BigInt(100));
    const bobCount = await bobActor.increment();
    expect(bobCount).toEqual(BigInt(101));

    // Assert: Verify isolation - each principal maintains their own counter
    expect(await aliceActor.get_count()).toEqual(BigInt(2));
    expect(await bobActor.get_count()).toEqual(BigInt(101));
  });

  it("should handle anonymous principal operations", async () => {
    // Execute: Default actor is anonymous, verify it works correctly
    await actor.set_count(BigInt(42));
    const incrementedValue = await actor.increment();

    expect(incrementedValue).toEqual(BigInt(43));
    expect(await actor.get_count()).toEqual(BigInt(43));
  });
});
