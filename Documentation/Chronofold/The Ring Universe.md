# The Ring Universe: A Relational Framework for Emergent Spacetime

## 1. Core Philosophy

This model adheres strictly to the **Principle of Maximal Simplicity**. It assumes no background space, no global clock, no continuous mathematical variables, and no random number generators. All spatial geometry and temporal flow emerge purely from discrete, localized graph rewrites driven by deterministic rules.

## 2. Fundamental Entities

### 2.1 The Monad

The Monad is the fundamental actor of the universe. It contains no internal state variables (no stress, no phase, no probability scalars). It consists solely of:

- **The Ring:** An ordered, cyclic array of Connectors. The length of the Ring defines the Monad's local mass/tension.
    
- **The Dial:** An index pointer indicating exactly one active Connector in the Ring at any given moment.
    

### 2.2 The Connector

The Connector is an undirected edge linking exactly two Monads. It acts as the medium of topological exchange. Because Monads reference the Connector (rather than pointing directly to each other), dangling pointers are mathematically impossible during graph rewrites.

A strict constraint exists: between any two unique Monads, there can exist a maximum of one Connector. Additionally, a Monad cannot connect to itself (no self-loops).

## 3. The Engine of Time (The Tick-Tock Architecture)

Time does not exist as a global continuum; it is the discrete sequence of local events. To process these events without introducing a "topological wind" or a centralized manager, the universe relies on a synchronized, three-phase event engine utilizing two alternating spatial queues and one action queue.

- **Advance-Queue A & Advance-Queue B:** These handle the flow of time (the search for alignment).
    
- **The Act-Queue:** This handles the mutation of space (the topological rewrites).
    

The Engine strictly cycles through three deterministic phases:

**Phase 1: The Tick (Search)** The Engine exclusively pops instructions from the active Advance-Queue (e.g., Queue A) until it is completely empty.

- For each instruction, the target Monad moves its Dial one step clockwise.
    
- If mutual alignment is achieved across a Connector, an `[Act]` event is generated and pushed to the **Act-Queue**.
    
- If there is no alignment (a Miss), the Monad pushes its next `[Advance]` instruction to the standby Advance-Queue (Queue B).
    

**Phase 2: The Resolve (Mutate)** Once Phase 1 finishes, the Engine shifts entirely to the **Act-Queue**.

- It pops and executes every pending Atomic Rewrite (Split, Weave, or Snap) based on the local geometry of the interacting Monads.
    
- Upon completing an action, the surviving Monads push their next `[Advance]` instructions to the standby Advance-Queue (Queue B) so they may participate in the next temporal cycle.
    
- _(Note: The deterministic resolution of simultaneous, conflicting Act-Events is currently under development)._
    

**Phase 3: The Tock (Swap)** Once the Act-Queue is entirely empty, the current spatial frame is resolved. The Engine swaps the active and standby queues (Queue B becomes the new Queue A), and the cycle loops back to Phase 1.

## 4. The Event Loop & The Handshake

When `[Advance A]` is popped from the active Advance-Queue, Monad A executes the following sequence:

### Step 1: The Advance

Monad A moves its Dial one step clockwise to the next Connector in its Ring.

### Step 2: The Alignment Check

Monad A checks the peer on the other side of the active Connector (Monad B).

- **The Miss:** If Monad B's Dial is _not_ pointing at this shared Connector, no interaction occurs. Monad A simply pushes its next `[Advance A]` instruction to the **standby Advance-Queue**.
    
- **The Handshake:** If Monad B's Dial _is_ pointing at this shared Connector, mutual alignment is achieved. Instead of executing the physics immediately, Monad A (as the Initiator) generates an `[Act]` event and pushes it to the **Act-Queue**.
    

## 5. Atomic Rewrites (The Physics)

During **Phase 2 (The Resolve)**, when an `[Act(Monad A)]` event is popped from the Act-Queue, Monad A evaluates its current local state against a strict priority hierarchy to execute exactly one action.

### Priority 1: Split (Generation / Pressure Release)

- **Selector:** The Monad is under critical geometric tension.
    
- **Condition:** Monad A's Ring length is $\ge 6$.
    
- **Consequence:** 1. A new Connector is forged.
    
    2. Monad A cleaves itself in half into two separate Monads (A1 and A2), sharing the new Connector between them. The split is made between the active connector and the next, and that is where the new connector is positioned.
    
    3. A2 sets its Dial to point at the newly shared Connector.
    
    4. **Continuation:** `[Advance A1]` and `[Advance A2]` are pushed to the **standby Advance-Queue**.
    

### Priority 2: Weave (Gravity / Topological Folding)

- **Selector:** The Monad has available capacity and seeks novel geometry.
    
- **Condition:** Monad A's Ring length is $< 6$. Monad A looks at its _next_ sequential Connector, which leads to Target Monad C. Monad B (the peer) must _not_ already have a Connector to Monad C.
    
- **Consequence:**
    
    1. A new Connector $c_{BC}$ is created between Monad B and Target Monad C.
        
    2. Monad B inserts $c_{BC}$ into its Ring exactly after its connection to Monad A.
        
    3. Target Monad C inserts $c_{BC}$ into its Ring exactly after its connection to Monad A.
        
    4. **Continuation:** `[Advance A]` is pushed to the **standby Advance-Queue**. _(Note: Monad B is left undisturbed, but its Dial is naturally set to land on the newly forged $c_{BC}$ on its next `Advance` turn, directing the causal flow)_.
        

### Priority 3: Snap (Entropy / Decay)

- **Selector:** Local space is deadlocked, or a connection is a dead end.
    
- **Condition:** Monad A cannot Split (length $< 6$), AND the Weave fails for one of two reasons:
    
    1. **Redundancy:** Monad A presents Target C, but Monad B already has a Connector to Monad C.
        
    2. **Isolation:** Monad A (or Monad B) has a Ring length of exactly 1, meaning there is no "next" Target to present.
        
- **Consequence:**
    
    1. The active Connector between Monad A and Monad B is destroyed and removed from both Rings.
        
    2. **Garbage Collection:** If Monad A or Monad B's Ring length drops to $0$ (which happens if it triggered the Isolation condition), that Monad instantly evaporates from the universe. Any pending event belonging to the deleted monad in the Act-Queue is either removed immediately or ignored when scheduled for execution.
        
    3. **Continuation:** `[Advance]` is pushed to the **standby Advance-Queue** for any Monad that survived the Snap.

## 6. Initialization (The Big Bang)

To prevent the universe from instantly deadlocking and Snapping into nothingness, the simulation cannot begin with a binary pair.

The universe is initialized with a **Primordial Seed**: a simple, open loop of **7 Monads** (a 7-Ring).

- **The Geometry:** Each Monad starts with exactly 2 Connectors (Left and Right).
    
- **Queue Initialization:** * **Advance-Queue 1 (Active):** Seeded with exactly one `[Advance]` instruction for each of the 7 primordial Monads.
    
    - **Act-Queue:** Initialized completely empty.
        
    - **Advance-Queue 2 (Standby):** Initialized completely empty.
        
- **The Spark:** The Engine boots up and immediately enters **Phase 1 (The Tick)**, pulling from Advance-Queue 1. This ensures the initial Monads advance their Dials to search for the first alignments, organically populating the Act-Queue and Advance-Queue 2 for the subsequent phases.
    
- **Cosmic Inflation:** This specific geometry guarantees enough internal volume for the network to rapidly Weave, hit the Split threshold ($\ge 6$), and trigger self-sustaining cosmic inflation.