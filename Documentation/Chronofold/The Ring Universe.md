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

## 3. The Engine of Time

Time does not exist globally; it is the sequence of events. To process these events without introducing a "topological wind" or a centralized manager, the universe relies on a **Global FIFO Queue**.

- The queue exclusively contains one type of instruction: `[Advance Monad_ID]`.
    
- The engine pops the first instruction, executes the local physics logic for that specific Monad, and appends the resulting continuations to the back of the queue.
    

## 4. The Event Loop & The Handshake

When `[Advance A]` is popped from the queue, Monad A executes the following atomic sequence:

### Step 1: The Advance

Monad A moves its Dial one step clockwise to the next Connector in its Ring.

### Step 2: The Alignment Check

Monad A checks the peer on the other side of the active Connector (Monad B).

- **The Miss:** If Monad B's Dial is _not_ pointing at this shared Connector, no interaction occurs. Monad A simply pushes `[Advance A]` to the back of the queue.
    
- **The Handshake:** If Monad B's Dial _is_ pointing at this shared Connector, mutual alignment is achieved. Monad A (as the Initiator) triggers an Atomic Rewrite based on the local geometry.
    

## 5. Atomic Rewrites (The Physics)

Upon a successful Handshake, Monad A evaluates its state against a strict priority hierarchy to execute exactly one action.

### Priority 1: Split (Generation / Pressure Release)

- **Selector:** The Monad is under critical geometric tension.
    
- **Condition:** Monad A's Ring length is $\ge 6$.
    
- **Consequence:** 1. A new Connector is forged.
    
    2. Monad A cleaves itself in half into two separate Monads (A1 and A2), sharing the new Connector between them. The split is made between the active connector and the next, and that is where the new connector is positioned.
    
    3. A2 sets its Dial to point at the newly shared Connector.
    
    4. **Continuation:** `[Advance A1]` and `[Advance A2]` are pushed to the queue.
    

### Priority 2: Weave (Gravity / Topological Folding)

- **Selector:** The Monad has available capacity and seeks novel geometry.
    
- **Condition:** Monad A's Ring length is $< 6$. Monad A looks at its _next_ sequential Connector, which leads to Target Monad C. Monad B (the peer) must _not_ already have a Connector to Monad C.
    
- **Consequence:**
    
    1. A new Connector $c_{BC}$ is created between Monad B and Target Monad C.
        
    2. Monad B inserts $c_{BC}$ into its Ring exactly after its connection to Monad A.
        
    3. Target Monad C inserts $c_{BC}$ into its Ring exactly after its connection to Monad A.
        
    4. **Continuation:** `[Advance A]` is pushed to the queue. _(Note: Monad B is left undisturbed, but its Dial is naturally set to land on the newly forged $c_{BC}$ on its next `Advance` turn, directing the causal flow)._
        

### Priority 3: Snap (Entropy / Decay)

- **Selector:** Local space is deadlocked, or a connection is a dead end.
    
- **Condition:** Monad A cannot Split (length $< 6$), AND the Weave fails for one of two reasons:
    
    1. **Redundancy:** Monad A presents Target C, but Monad B already has a Connector to Monad C.
        
    2. **Isolation:** Monad A (or Monad B) has a Ring length of exactly 1, meaning there is no "next" Target to present.
        
- **Consequence:**
    
    1. The active Connector between Monad A and Monad B is destroyed and removed from both Rings.
        
    2. **Garbage Collection:** If Monad A or Monad B's Ring length drops to $0$ (which happens if it triggered the Isolation condition), that Monad instantly evaporates from the universe. Any event belonging to the deleted monad is either removed immediately or when it is scheduled for execution.
        
    3. **Continuation:** `[Advance]` is pushed to the queue for any Monad that survived the Snap.

## 6. Initialization (The Big Bang)

To prevent the universe from instantly deadlocking and Snapping into nothingness, the simulation cannot begin with a binary pair.

The universe is initialized with a **Primordial Seed**: a simple, open loop of **7 Monads** (a 7-Ring).

- Each Monad starts with exactly 2 Connectors (Left and Right).
    
- The FIFO queue is seeded with one `[Advance]` instruction for each of the 7 Monads.
    
- This specific geometry guarantees enough internal volume for the network to rapidly Weave, hit the Split threshold ($\ge 6$), and trigger self-sustaining cosmic inflation.