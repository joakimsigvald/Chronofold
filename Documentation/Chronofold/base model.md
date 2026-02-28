# The Fluid Spacetime Engine
**Goal:** To derive a self-saturating, 4D macroscopic world model (a zero-energy vacuum state capable of supporting persistent "Braid and Loop" topological particles) entirely from the simplicity of relational sequences and pairs, without relying on a rigid coordinate grid or global clock.

## 1. Core Primitives
Space and Time do not exist as containers. They emerge from the interactions of the fundamental unit: **The Monad**.

A Monad is completely defined by two data structures:
* **The History (Proto-Time):** An immutable, strictly ordered ledger of every event the Monad has participated in. It represents the causal timeline of the entity.
* **The Active Horizon (Proto-Space):** A mutable, Least Recently Used (LRU) queue containing the IDs of other Monads. It defines who the Monad is currently topologically "entangled" with. 

## 2. The Mechanics of Reality
Because there is no global clock or spatial coordinate system, reality updates via localized, asynchronous interactions.

### The Global Loop (Relativistic Safety)
To simulate asynchronous reality on a synchronous CPU without introducing a spatial bias, every global tick must begin with a **Fisher-Yates Shuffle**. The array of Monads is randomized before execution to act as quantum jitter, ensuring no node gets absolute priority.

### The Handshake Protocol (2-Phase Commit)
Events occur via an Asynchronous Propose/Resolve cycle to prevent topological deadlocks:
1.  **Signal Phase:** Monad A decides to act, targets Monad B from its Active Horizon, and sends a proposal.
2.  **Receive Phase:** Monad B evaluates. If idle, it accepts (Success). If already engaged, it rejects (Failure/Bounce). Mutual proposals immediately lock and resolve.

## 3. Event Types (Graph Rewriting)
When a Handshake succeeds, the graph rewrites itself using one of two methods:

* **The Internal Handshake (Gravity / Weaving):** A Monad connects with an existing ID in its Active Horizon. The two Monads sync their phases, log the event in their Histories, and exchange a subset of their Active Horizons (Triad Closure). This process folds branches together, cross-linking the network into a dense 2D or 3D topological mesh.
* **The Genesis Handshake (Dark Energy / Expansion):** A Monad spawns a brand new Monad. The old Monad adds the new one to its Horizon, and the new one only knows the old one. This continuously expands the universe without shattering it into disconnected pieces.

## 4. The Thermodynamic Drivers (State Variables)
The type of event a Monad pursues, and how it prunes its Active Horizon, is governed by two continuous, orthogonal internal scalars bounded between $0$ and $1$.

### Anxiety ($A$): The Drive to Bind
Anxiety measures topological isolation. It is driven purely by the success or failure of proposals. High Anxiety forces a Monad to shrink its allowed Active Horizon, increasing the probability of matching with known, reliable neighbors. Let $\alpha$ be the learning rate:
* **On Failure:** The Monad becomes more anxious/isolated. 
    $$A \leftarrow A + \alpha(1 - A)$$
* **On Success:** The Monad relieves its anxiety.
    $$A \leftarrow A(1 - \alpha)$$

### Boredom ($B$): The Drive to Expand
Boredom measures topological stagnation. It evaluates the recency index ($n$) of a successful connection within the LRU queue. High Boredom triggers a Genesis Handshake to inject novel geometry into the local space. Let $\beta$ be the sensitivity rate:
* **On Failure:** Unchanged (chaos is not boring).
* **On Success:** Boredom blends with the exponential recency of the connection.
    $$B \leftarrow (1 - \beta)B + \beta(2^{-n})$$
*(Note: Connecting to a recently used node where $n=0$ drives $B$ toward $1$. Connecting to an old node where $n$ is large drives $B$ toward $0$.)*