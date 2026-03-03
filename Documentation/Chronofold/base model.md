# The Fluid Spacetime Engine
**Goal:** To derive a self-saturating, 4D macroscopic world model (a zero-energy vacuum state capable of supporting persistent "Braid and Loop" topological particles) entirely from the simplicity of relational sequences and pairs, without relying on a rigid coordinate grid or global clock.
## 1. Core Primitives
Space and Time do not exist as containers. They emerge from the interactions of the fundamental unit: **The Monad**.

A Monad is completely defined by its **Context** and its **State**.
### The Context (Topology & Causality)
* **The History (Proto-Time):** An immutable, strictly ordered ledger of every event the Monad has participated in. It represents the causal timeline of the entity.
* **The Active Horizon (Proto-Space):** A mutable, Least Recently Used (LRU) queue containing the IDs of other Monads. It defines who the Monad is currently topologically "entangled" with. 
* **The MaxCapacity (M)**: The maximum count of the Active horizon (N), where M >= N.
### The State (Psychology & Thermodynamics)
The state consists of two continuous scalars bounded between 0 and 1.
* **Boredom ($B$):** The intolerance for repetition. High boredom expands the Active Horizon and increases the probability of spawning new monads (Expansion/Dark Energy).
* **Anxiety ($A$):** The intolerance for chaos. High anxiety shrinks the Active Horizon, increasing the probability of pairing with familiar, reliable monads (Gravity/Binding).

Two components of the state can be derived from the first two:
* **Happiness (H)**: Not a driver for action but a target to aim for. The system will try to optimize for maximal happiness at all times. Happiness is calculated as H = Sqrt((1 - A)(1 - B))
* **Curiosity (C)**: Curiosity is the driver for exploring the unknown. It is calculated as C = Sqrt((1 - A)B). Curiosity is proportional with boredom but the capacity for curiosity is limited by anxiety.

---
## 2. The Mechanics of Reality
Because there is no global clock or spatial coordinate system, reality updates via localized, asynchronous interactions.
### The Double-Buffered Execution Loop
To simulate asynchronous reality without introducing deterministic spatial biases (a "wind"), the engine process monads in a Double-Buffered FIFO Queue.
Each monad is handled in turn.
If the monad doesn't signal, it is moved to a handled queue. 
If it signals, it is moved to a waiting queue.
the waiting queue is processed after the signaling phase to see which signal is resolved. Regardless if a handshake is successful or rejected, the signaling monad goes to the handled queue. The same goes for signaling monads that receive and accept a signal while it is still in the waiting queue.

The handled queue becomes the next round's main queue.
### The Handshake Protocol (2-Phase Commit)
Events occur via an Asynchronous Propose/Resolve cycle to prevent topological deadlocks:
1. **Signal Phase:** Monad A decides to act. Based on its current Curiosity, it selects a target Monad B from its Active Horizon (LRU) and sends a proposal. The index of the monad to signal from the ActiveHorizon is calculated as {Floor(C * M) | C < 1, M-1 | C = 1}. The higher the curiosity, the farther out the monad will reach.
2. **Receive Phase:** Monad B evaluates the proposal. 
    * If idle, it accepts (Success). 
    * If already locked in a resolved event, it rejects (Bounce). 
    * **Deadlock Break:** If Monad B has currently dispatched its own signal but has not yet been acknowledged, it will immediately accept the incoming proposal, drop its signal, and move from the waiting to the handled queue. This elegantly guarantees the resolution of circular deadlocks. It will also ensure that two monads signaling each other will resolve the event (one of the signals will be resolved while the opposite signal is deleted).

---
## 3. Event Types (Graph Rewriting)
When a Handshake succeeds, the universe physically updates using one of two methods:

* **The Internal Handshake (Gravity / Weaving):** A Monad connects with an existing ID in its Active Horizon. The two Monads sync, log the event, and put each other first in their respective ActiveHorizon. If any of them has room to expand their horizon (M > N) they pick the first (most recent) monad from the other's ActiveHorizon that is not already in their own ActiveHorizon. ActiveHorizon can only be expanded with one monad each tick/round. We call this a Triad Closure - it folds branches together, cross-linking the network into a dense 2D or 3D mesh.
* **The Genesis Handshake (Dark Energy / Expansion):** If a monad selects an index outside of its ActiveHorizon, it spawns a new monad and sends a signal to it. Since it is not yet in the ActiveHorizon its distance will be considered infinite, which will affect the boredom update as we will see later). When the new monad accepts it will be added to the beginning of the ActiveHorizon (most recent). The new monad is initialized with (M=1,A=0,B=0) and added to the end of the handled queue (as opposed to human babies, newborn monads are perfectly happy).

---
## 4. The Thermodynamic Drivers (Update Rules)
A Monad's decision to act, who it targets, and how it modifies its Horizon are dictated by the continuous drift and event-triggered jumps of its internal scalars. 

Let $N$ be the current size of the Active Horizon, and $x$, $y$, $\alpha$, and $\beta$ be calibration constants.
### The Continuous Drift (Tick Updates)
Every time a Monad is evaluated in the global loop, its tension naturally rises.
* **Boredom increases:** $B \leftarrow B + x \frac{1 - B}{N}$
* **Anxiety increases:** $A \leftarrow A + y(1 - A)$
* **The Action Trigger:** The Monad will choose to enter the Signal Phase if $H < t_h$ (the threshold for action).
### The Event Resolution (Handshake Updates)
If a Monad attempts an action, the success or failure of that handshake instantly alters its state.

**Anxiety ($A$): The Drive to Bind**
Driven purely by the success or failure of the proposal.
* **On Failure (Bounce):** The Monad becomes more isolated. $A \leftarrow A + \alpha(1 - A)$
* **On Success:** The Monad relieves its isolation. $A \leftarrow A(1 - \alpha)$

**Boredom ($B$): The Drive to Expand**
Evaluates the recency index ($n$) of the partner within the LRU queue upon a successful connection.
* **On Failure:** Unchanged (chaos is not boring).
* **On Success:** Blends with the exponential recency of the connection. $B \leftarrow (1 - \beta)B + \beta(2^{-n})$
*(Note: Connecting to a recently used node where $n=0$ drives $B$ toward 1. Connecting to an old, novel node where $n$ is large drives $B$ toward 0).**

**ActiveHorizon**
After the signal and resolve phases, monads will update the size of its ActiveHorizon based on its state:
* If boredom is above a threshold, $t_{b}$, MaxCapacity, M, is increased by 1.
* If anxiety is above a threshold, $t_{a}$, MaxCapacity, M, is decreased by 1. If N > M, the last monad will be dropped from ActiveHorizon, making N = M. M can never drop below 1.
## The initial state (Big bang)
Two monads and an event is created initially. Both monads have M=N=1 with each other in their ActiveHorizons and the initial event in their history.