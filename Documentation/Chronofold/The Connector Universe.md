# The Connector Universe

### A Relational Framework for Emergent Spacetime and Quantum Mechanics

## I. Core Philosophy

This model adheres strictly to the **Principle of Maximal Simplicity**. It assumes no background space, no global clock, no continuous mathematical variables, and no random number generators. All spatial geometry, temporal flow, and quantum phenomena emerge purely from discrete, localized graph rewrites.

## II. Ontology: The Graph Elements

The universe consists entirely of a set of Vertices $V$ (Ports) and a set of directed Edges $C$ (Connectors).

**1. The Connectors ($C$)**

Each connector $c \in C$ is an ordered pair of vertices $(v_{tail}, v_{head})$ representing a directional link. Every connector is permanently assigned one of three Roles:

- **Ring ($C_R$):** Defines the structural boundary of a Monad.
    
- **Bond ($C_B$):** Defines the topological space between Monads.
    
- **Dial ($C_D$):** Defines the active computation state of a Monad.
    

**2. The Vertices ($V$)**

A vertex $v \in V$ is a stateless intersection point. To ensure strict structural integrity and a fully contiguous universe, every vertex is bounded by these degree rules:

- Exactly 1 incoming Ring connector and 1 outgoing Ring connector.
    
- **Exactly 1 Bond connector.** (There are no empty ports; bonds and their anchor vertices exist symbiotically).
    
- Maximum 1 Dial connector.
    

## III. Entities: The Emergent Monad

A **Monad** $M$ is strictly defined as an emergent, localized subgraph consisting of:

- A closed, directed cycle of Ring connectors ($N = |C_R| \ge 2$).
    
- Exactly one internal Dial connector $d \in C_D$.
    
- A fundamental law: Between any two distinct Monads $M_A$ and $M_B$, there can exist at most one Bond connector.
    

## IV. Kinematics: The Dial Engine (Local Time)

Time progresses via local asynchronous updates organized into a strictly prioritized, two-phase execution cycle to prevent race conditions. The topological order of the Ring connectors establishes a strictly local "Clockwise" ($CW$) direction. Let $next(v)$ be the adjacent vertex reached by traversing the outgoing Ring connector from $v$.

**Phase 1: The Dial Phase**

For a Monad's Dial $d = (v_{tail}, v_{head})$, the **Advance Protocol** operates locally:

- **Check:** Evaluate $next(v_{head})$.
    
- **Move:** If $next(v_{head}) \neq v_{tail}$, the head advances: $v_{head} \leftarrow next(v_{head})$.
    
- **Swap:** If $next(v_{head}) = v_{tail}$ (the Head is blocked by its own Tail), the Dial reverses direction: $d \leftarrow (v_{head}, v_{tail})$. In effect, Move and Swap ensures that the dial head advances clockwise one step at a time without interruption or jumps.
    

_Corollary: If $N=2$, the Dial is in a perpetual **Temporal Lock**, swapping head back and forth between the two vertices_.

## V. Dynamics: The Interaction Rules

**Phase 2: The Bond Phase**

When a Bond connector $b$ (anchored on Monads $X$ and $Y$) shares both of its vertices with the heads of two different Dials, the Bond becomes **Active**. It executes the Universal Imperative targeting the vertices at the _tail_ of each respective Dial (let's call these target vertices $t_1$ and $t_2$).

Because updates are asynchronous, the Active Bond blindly attempts to link the two Monads attached via the external bonds at $t_1$ and $t_2$. Let's call these target Monads $M_A$ and $M_B$. Only one of three outcomes can occur:

**1. The Snap (Annihilation)**

- **Condition:** $M_A$ and $M_B$ are the exact same Monad (a self-loop paradox).
    
- **Action:** The Weave deadlocks. The Active Bond $b$ is destroyed, releasing the topological strain. Because every vertex must have a bond, the two vertices that anchored $b$ on Monads $X$ and $Y$ also dissolve. Monads $X$ and $Y$ shrink by 1 ($N \leftarrow N - 1$).
    

**2. The Weave (Expansion)**

- **Condition:** $t_1$ and $t_2$ belong to distinct Monads $M_A$ and $M_B$, and no Bond currently exists between them.
    
- **Action:** A new Bond $b_{new} = (t_1, t_2)$ is forged. To accommodate this, a new ring vertex is inserted at $next(v_A)$ in $M_A$, and another at $next(v_B)$ in $M_B$. These newly inserted vertices anchor the new bond, effectively connecting $M_A$ with $M_B$ and permanently increasing the size of both Monads by 1 ($N \leftarrow N + 1$).
    

**3. The Promotion (Pinch-Off)**

- **Condition:** $t_1$ and $t_2$ belong to distinct Monads $M_A$ and $M_B$, but a Bond _already exists_ between them, creating an illegal double-link.
    
- **Action:** 1. A new Monad $M_C$ must be generated between them to resolve the violation.
    
    2. The Weave attempts to execute first, forging the forbidden bond and temporarily leaving $M_A$ and $M_B$ with one extra vertex ($N + 1$).
    
    3. The new forbidden bond is immediately promoted to become the Dial connector of the new Monad $M_C$. The two newly inserted ring vertices are pinched off from $M_A$ and $M_B$ and promoted to become the initial ring vertices of $M_C$.
    
    4. One _additional_ ring connector is then pinched off from $M_A$ and promoted to become the Bond connector linking $M_A$ and $M_C$. The exact same happens with $M_B$.
    
    5. The rings of $M_A$ and $M_B$ close the gap left by the missing vertices. If a Dial was pointing at a removed vertex, it is automatically adjusted to the nearest free vertex.
    
    6. Because of this sequence (gain 1, donate 1 to ring, donate 1 to bond), $M_A$ and $M_B$ reduce their overall size by a net of 1. If this leaves either Monad with exactly 1 vertex left ($N=1$), they pop out of existence.
    

**4. Cascading Delete**

- If a Monad is destroyed, the Bond connectors attached to it are also destroyed, along with the anchor vertices on the corresponding peer Monads.
    
- If the loss of that vertex leaves a peer Monad with $N=1$, it too pops out of existence. This deletion cascades through the network until it reaches a stable boundary where all remaining Monads have at least $N=2$.

- Any time a vertex is deleted in a monad, the internal dial has to adjust so it is anchored at existing vertices at both ends
    
## VI. Emergent Quantum Mechanics

In a purely asynchronous engine, concurrent local updates can create conflicting graph topologies. Instead of enforcing an absolute sequence, the universe accommodates parallel valid actions.

- **Localized Superposition:** The local geometry branches into multiple concurrent states, representing all valid Weave applications. A "Wave" of topological mutation propagates through the graph.
    
- **Constructive Interference (Merge):** If the divergent branches naturally lead to the exact same macroscopic geometric shape after a few local updates, the branches mathematically merge. The wave amplifies, creating stable, dense topological structures.
    
- **Destructive Interference (Collapse):** If the branches contradict each other, they create topological knots. These deadlocks naturally trigger an **Emergent Snap**, deleting the impossible geometry. The superposition physically collapses, pruning the Multiway Graph back into a single, definitive timeline.
    

## VII. Initialization (The Big Bang)

To prevent the universe from instantly deadlocking and Snapping into nothingness, the simulation cannot begin with a simple binary pair.

- **The Primordial Seed:** The universe is initialized as a simple, open loop of **5 Monads** (a 5-Ring).
    
- **The Geometry:** Each Monad starts strictly at $N=2$, permanently locked in a state of Dial swapping.
    
- **Cosmic Inflation:** This specific hyper-dense geometry guarantees that the very first asynchronous Bond activations will immediately trigger topological short circuits (Promotions). The 5-Ring violently beads and expands, triggering self-sustaining cosmic inflation.