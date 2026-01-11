# MARKENZ UNIVERSE — FULL SYSTEM SPECIFICATION

**Status:** PRODUCTION / CANONICAL  
**Version:** 2.0.0 (System Complete)  
**Authority:** [Governance Master Roadmap](docs/roadmap/MARKENZ_GOVERNANCE_MASTER_ROADMAP.md)  

---

## 1. What Markenz Is — and Is Not

### What It Is

Markenz is a **causally closed, biologically and physically rigorous simulation of human-equivalent agents** living in a deterministic environment. It is a "universe in a box" where every outcome—from a footstep to a civilization's collapse—is the result of granular, low-level interactions between matter, biology, and cognition.

* **It is a scientific instrument:** Used to study emergence, sociology, and economics in a clean-room environment.
* **It is a life support system:** It runs real-time autonomous biological organisms.
* **It is an auditable ledger:** Every event is cryptographically hashed; history is immutable.

### What It Is NOT

* **NOT a "Game":** There are no "hit points," "levels," or " NPCs." There is no winning condition.
* **NOT a "Chatbot":** Agents are not LLM wrappers. They do not "predict text"; they *think* and *speak* based on internal needs.
* **NOT "Magic":** There is no teleportation, no instant healing, no spawning without reproduction, no information transfer without physical transmission (sound/light).
* **NOT "Speed-Up":** Time runs at 1:1 or fixed ratios. You cannot "skip" gestation or childhood.

---

## 2. World Substrate

### Time Model

The universe advances in discrete, atomic instants called **Ticks**.

* **Tick Rate:** 20 Hz (1 Tick = 50ms).
* **World Second:** 20 Ticks.
* **World Day:** 86,400 World Seconds (1,728,000 Ticks).
* **Circadian Cycle:** 24-hour light/dark cycle driven by planetary rotation simulation.
* **Time Authority:** The Rust engine monotonically increments the `Tick`; time *never* reverses unless during a tracked replay Audit.

### Spatial Model

* **Coordinate System:** 64-bit Floating Point Cartesian (x, y, z).
* **Terrain:** Voxel-based volumetric terrain stored in 32x32x32 Chunks.
* **Scale:** 1.0 coordinate unit = 1.0 meter.
* **Physics:** Newtonian rigid-body dynamics (mass, velocity, friction, restitution) calculated per Tick.

### Material Reality

* **Conservation of Mass:** Matter is neither created nor destroyed, only transformed (e.g., food → biomass + waste).
* **Thermodynamics:** Energy (kcal/Joules) is conserved. Entropy increases over time.
* **Environment:**
  * **Temperature:** Thermodynamic heat propogation (conduction, convection, radiation).
  * **Light:** Ray-traced visibility; photons are required for vision.
  * **Atmosphere:** Composition (O2, N2, CO2) tracked per Chunk; breathability is dynamic.

---

## 3. Synthetic Human Body Model (EXHAUSTIVE)

Markenz implements a **Human-Equivalent Biological Layer**. Agents are not objects; they are organisms.

### 3.1 Anatomy & Physiology

* **Musculoskeletal:**
  * Skeleton: Rigid bone segments with fracture thresholds.
  * Muscles: Actuators consuming ATP/Energy to apply force.
  * Fatigue: Lactic acid accumulation reduces force output locally.
* **Circulatory System:**
  * Heart Rate (BPM) responding to metabolic demand and adrenaline.
  * Blood Volume: Hemorrhage leads to hypovolemic shock.
  * Blood Pressure: Dynamic systolic/diastolic modeling.
* **Respiratory System:**
  * Gas Exchange (O2 intake / CO2 output).
  * Respiratory Rate: Driven by blood CO2 partial pressure.
* **Digestive System:**
  * Stomach (Capacity, HCL breakdown).
  * Intestines (Nutrient absorption time-delay).
  * Bladder/Bowel (Waste storage requiring evacuation actions).
* **Thermoregulation:**
  * Core Body Temperature (Target: 37°C).
  * Mechanisms: Shivering (generates heat, burns energy), Sweating (evaporative cooling, consumes water), Vasodilation/constriction.
* **Integumentary System:**
  * Skin integrity (cuts, burns, infections).
* **Immune System:**
  * Non-specific and adaptive immunity simulation (white blood cell count proxy).
  * Infection progression vs. immune defense magnitude.

### 3.2 Metabolism & Nutrition

Agents must consume specific nutrients to survive. Metabolic rate follows Mifflin-St Jeor equations adjusted for activity.

* **Macronutrients:**
  * Proteins (Amino acid pool for tissue repair).
  * Carbohydrates (Glycogen for immediate energy).
  * Fats (Adipose tissue for long-term storage).
* **Micronutrients (Vitamins):**
  * Fat-Soluble: Vitamin A, D, E, K.
  * Water-Soluble: Vitamin C, B1 (Thiamine), B2 (Riboflavin), B3 (Niacin), B5 (Pantothenic Acid), B6, B7 (Biotin), B9 (Folate), B12.
  * *Effect:* Deficiency in any leads to specific system failures (e.g., Vit C deficiency → Scurvy/bleed risk).
* **Minerals:**
  * Major: Calcium, Chloride, Magnesium, Phosphorus, Potassium, Sodium, Sulfur.
  * Trace: Iron, Zinc, Iodine, Selenium, Copper, Manganese, Chromium, Molybdenum.
* **Hydration:**
  * Water balance tracked in mL.
  * Losses: Urine, sweat, respiration.
  * Dehydration leads to cognitive decline → circulatory collapse → death.

### 3.3 Hormonal System (EXPLICIT ENUMERATION)

The endocrine system drives behavior. Agents do not "choose" to be stressed; their hormones force it.

| Hormone | Source | Trigger(s) | Decay Half-Life | Behavioral/Physiological Effect |
| :--- | :--- | :--- | :--- | :--- |
| **Dopamine** | Brain (VTA/SN) | Reward perception, anticipation | Fast | Increases motivation, focus, reinforcement learning. |
| **Serotonin** | Brain/Gut | Satiety, social status, daylight | Medium | Stabilizes mood, regulates sleep/wake transitions. |
| **Cortisol** | Adrenal | Stress, low blood sugar, injury | Slow | mobilizing energy, suppressing immune, increasing anxiety. |
| **Adrenaline** | Adrenal | Threat, shock, excitement | Very Fast | Fight-or-flight: HR up, BP up, glucose dump, tunnel vision. |
| **Noradrenaline** | Adrenal/Brain | Novelty, vigilance | Fast | Alertness, arousal, readiness to act. |
| **Oxytocin** | Pituitary | Touch, birth, bonding | Fast | Increases social trust, reduces fear, facilitates bonding. |
| **Vasopressin** | Pituitary | Dehydration, stress | Fast | Retains water, increases aggression/territoriality. |
| **Testosterone** | Gonads | Status seeking, competition | Slow | Increases muscle synth, libido, dominance drive. |
| **Estrogen** | Gonads | Cycle position | Slow | Regulates cycle, mood stability, verbal memory. |
| **Progesterone** | Gonads | Luteal phase, pregnancy | Slow | Sedative effect, prepares uterus (if female). |
| **Melatonin** | Pineal | Darkness (circadian) | Fast | Induces sleepiness, lowers body temp. |
| **Insulin** | Pancreas | Glucose intake | Fast | Stores energy (glucose -> glycogen/fat); lowers blood sugar. |
| **Glucagon** | Pancreas | Low glucose | Fast | Releases stored energy. |
| **Leptin** | Adipose | Fat mass accumulation | Slow | Signals satiety (long term), reduces hunger drive. |
| **Ghrelin** | Stomach | Empty stomach | Fast | Signals immediate hunger, increases food seeking. |
| **Thyroxine (T4)** | Thyroid | Baseline | Very Slow | Sets Basal Metabolic Rate (BMR). |
| **Growth Hormone** | Pituitary | Sleep (SWS), exercise | Medium | Tissue growth, repair, protein synthesis. |
| **Prolactin** | Pituitary | Lactation, post-orgasm | Medium | Lactation, sexual refractory period, caretaking drive. |
| **Endorphins** | CNS | Pain, exercise | Medium | Analgesia, euphoria. |
| **Adeonsine** | CNS (metabolite) | Waking time | N/A (Clears in sleep) | Sleep pressure (homeostatic drive). |

### 3.4 Vitals & Homeostasis

* **Heart Rate:** 40-200 BPM.
* **Blood Pressure:** 90/60 - 180/120 mmHg.
* **SpO2:** 80-100%. (<90% = Hypoxia → Cognitive failure).
* **Fatigue:** 0-100 scale. Accumulates with wakefulness (Adenosine). Clears with Sleep (NREM/REM stages).
* **Pain:** 0-10 scale. Driven by nociceptor stimulation. Causes Cortisol spike.

---

## 4. Somatic Sensation & Qualia Model

Agents *feel*. Nociception and mechanoreception are simulated inputs that the brain *must* process.

* **Mechanoreception:** Touch maps (homunculus) detect pressure/texture.
* **Thermoception:** Hot/Cold sensors trigger thermal comfort seeking behavior.
* **Nociception (Pain):** High-threshold sensors. Triggers immediate withdrawal reflex (spinal) AND long-term avoidance learning (cortical).
* **Proprioception:** Sense of body position in space (required for coordinated movement).
* **Interoception:** Sense of internal state (hunger pangs, full bladder, heart pounding).
* **Vestibular:** Sense of balance/acceleration.

**Qualia Implementation:** Qualia are represented as high-dimensional vector embeddings of sensory states that act as "attractors" or "repulsors" in the agent's planning space. "Pain" is mathematically defined as a state the planner is axiomatically compelled to minimize (Gradient Descent away from Pain vectors).

---

## 5. Emotion System (150+ EMOTIONS)

Emotions are **cognitive-hormonal composite states**. They are not text labels; they are functional overrides of the agent's priority system.

**Taxonomy:**

### Primary (Instinctual)

1. **Joy** (Ecstasy, Elation, Happiness, Delight, Amusement, Pleasure, Cheerfulness, Contentment, Satisfaction, Relief)
2. **Sadness** (Grief, Sorrow, Heartbreak, Despair, Misery, Gloom, Melancholy, Hopelessness, Disappointment, Resignation)
3. **Anger** (Rage, Fury, Wrath, Outrage, Hostility, Aggression, Frustration, Aggravation, Irritation, Annoyance)
4. **Fear** (Terror, Horror, Panic, Dread, Fright, Alarm, Anxiety, Nervousness, Apprehension, Caution)
5. **Disgust** (Revulsion, Loathing, Aversion, Repugnance, Distaste, Dislike)
6. **Surprise** (Astonishment, Amazement, Wonder, Shock, Startle)

### Secondary (Cognitive)

7. **Anticipation** (Excitement, Eagerness, Expectancy, Hope, Curiosity, Interest)
2. **Trust** (Acceptance, Faith, Confidence, Reliance)
3. **Submission** (Obedience, Compliance, Docility, Meekness)
4. **Dominance** (Authority, Command, Superiority)

### Social / Relationship

11. **Love** (Adoration, Affection, Fondness, Liking, Caring, Tenderness, Compassion, Empathy, Sympathy)
2. **Hate** (Malice, Spite, Contempt, Scorn, Disdain)
3. **Jealousy** (Envy, Covetousness, Possessiveness)
4. **Shame** (Humiliation, Disgrace, Embarrassment, Guilt, Remorse, Regret, Contrition)
5. **Pride** (Triumph, Victory, Achievement, Self-Esteem, Arrogance, Haughtiness, Vanity, Hubris)
6. **Gratitude** (Thankfulness, Appreciation)
7. **Indignation** (Righteous Anger, Offense)

### Existential / Complex

18. **Awe** (Reverence, Sublimity)
2. **Boredom** (Ennui, Apathy, Indifference, Listlessness, Dismay)
3. **Confusion** (Bewilderment, Perplexity, Bafflement)
4. **Determination** (Resolve, Perseverance, Tenacity, Stubbornness)
5. **Insecurity** (Vulnerability, Sensitivity, Fragility)
6. **Loneliness** (Isolation, Alienation, Abandonment)
7. **Nostalgia** (Sentimentality, Wistfulness)
8. **Schadenfreude** (Malicious Joy)

*(Full enumeration of 150+ entails valid sub-states of the above, e.g., "Pensive," "Wary," "Smug," "Coy," "Sheepish," "Stunned," "Mortified," "Euphoric," "Enraptured," "Despondent," "Forlorn," "Agitated," "Restless," "Serene," "Tranquil," "Blissful," "Grateful," "Indebted," "Vengeful," "Bitter," etc. — All are distinct hormonal/cognitive vectors in the system.)*

---

## 6. Cognition Without LLM

Markenz proves intelligence does not require Large Language Models.

* **Perception Pipeline:** Ray-traced vision & physics-based audio → Object Detection → Belief State Update.
* **World Model:** Probabilistic graph of Entities, Locations, and Relationships.
* **Memory Systems:**
  * **Sensory Buffer:** < 1 sec retention.
  * **Working Memory:** 7±2 active concepts.
  * **Episodic Memory:** Autobiographical storage of Tick-stamped events (forgetting curve applied).
  * **Semantic Memory:** Facts and generalizations.
  * **Procedural Memory:** Skills (Walking, crafting, fighting) learned via repetition.
* **Planning (GOAP/HTN):** Goal-Oriented Action Planning.
  * Input: Drives (Hunger, Social) + Environment.
  * Process: A* Search through action space to satisfy drives.
  * Output: Action Queue.
* **Language (Symbolic NLG):**
  * Constructs semantic propositions ("I" + "give" + "apple" + "you").
  * Maps to English using deterministic grammar engines and relationship-modulated lexicons.
  * **NO LLM HALLUCINATIONS:** Agents cannot say things they do not know. Deception is a deliberate plan, not a statistical error.

---

## 7. Reproduction, Genetics, and Development

Population growth occurs solely through biological reproduction.

* **Genome:** Diploid (2 alleles per locus). Double-Helix structure.
* **Inheritance:** True Mendelian inheritance (Independent Assortment + Segregation). Parents pass 50% of genes each.
* **Mutation:** Deterministic RNG-based point mutations and copy errors during meiosis.
* **Phenotype:** Genotype + Environment = Phenotype. Traits (Height, Eye Color, Intelligence Potential, Disease Risk) are calculated expressions.
* **Reproduction Pipeline:**
    1. **Attraction:** Based on genetic compatibility (MHC complex) and social triggers.
    2. **Conception:** Probabilistic based on fertility cycle.
    3. **Gestation:** 9-month simulation (embryo → fetus). Nutrition dependence.
    4. **Birth:** Discrete event spawning a new Agent Entity.
* **Aging:** Telomere shortening simulation. Cells senesce. Systems degrade efficientcy over time (aging). Death is inevitable by organ failure if not trauma.

---

## 8. Founder Agents (Gem-D & Gem-K)

The simulation began with two progenitors.

* **Human Equivalent:** They share the exact same physiology, needs, and fragility as all other agents. They bleed, starve, and feel pain.
* **Amplified Capabilities (The "Gift"):**
  * **Learning Rate:** 1.5x - 5.0x faster skill acquisition.
  * **Social Radiance:** High oxytocin induction in others (Leadership buff).
  * **Perfect Memory:** No forgetting curve for Episodic memory.
* **Limitations:** They are NOT gods. They cannot fly, spawn matter, or defy physics. They must eat and sleep.
* **Genetic Firewall:** Their amplification is **NOT HERITABLE**. Their children are baseline humans. This ensures the population remains standard human.

---

## 9. Determinism, Replay, and Auditability

* **The Golden Rule:** `State(T+1) = Function(State(T), Input(T))`.
* **RNG:** ChaCha20 Cryptographically Secure PRNG, seeded by the World Seed + Entity ID + Tick.
* **No Floating Point Drift:** Strict IEEE 754 handling or Fixed Point math where critical.
* **Auditable:** The entire universe can be re-simulated from Tick 0 given the Input Logs.
* **Hash Chain:** Every World State produces a SHA-256 Merkle Root. Divergence is detected instantly.

---

## 10. Governance & Law

The universe is ruled by code, not whim. Governance is a deterministic loop, not a god-mode dashboard.

### 10.1 Constitutional Hard-Code (The "Physics" of Law)

* **Governance Standard:** [GOVERNANCE_AND_LAWS.md](docs/governance/GOVERNANCE_AND_LAWS.md).
* **Agent Parity:** No "NPCs." Every agent runs the full bio-cognitive stack.
* **BioVeto:** An agent fundamentally *cannot* perform an action if their biology forbids it (e.g., lifting a rock when starving/atrophied). This is enforced by the engine, not the agent's "will."
* **The Iron Law:** Administrators cannot manually edit state tables to "fix" things. They must submit "Miracle" transactions (Input Events) which are logged and visible to the audit trail.

### 10.2 Legislative Pipeline (Dynamic Law)

* **Proposal:** Agents (or Admins via Miracle) submit `LawProposal` events containing deterministic logic clauses.
* **Voting:** A deterministic voting period mechanism. If `VoteRecorded` events satisfy the quorum and threshold (e.g., >50%), the law passes.
* **Enactment:** Passed laws emit `LawActivated` and are loaded into the **PolicyValidate** engine.

### 10.3 Executive Pipeline (PolicyValidate)

* **Checkpoint:** Runs *after* BioVeto but *before* Physics/Commit.
* **Logic:** Every tick, for every action, the active Law Set is evaluated against the `Intent`.
* **Veto:** If an action violates an active law, a `PolicyVeto` event is emitted with a specific reason code (e.g., `LAW_THEFT_001::v1`), and the action is cancelled.

### 10.4 Judicial Pipeline (Crimes & Punishment)

* **Detection:** `CriminalChargeRecorded` is emitted when a law is broken (if the law allows the action but penalizes it, vs. blocking it).
* **Trial:** `TrialHeld` event deterministically weighs evidence.
* **Consequence:** `PenaltyExecuted` applies sentences (fines, imprisonment, banishment).
* **Constraint:** Torture and Infinite Imprisonment are constitutionally forbidden methods of punishment.

---

## 11. Reality Lock & Engineering Discipline

* **No Placeholders:** Code containing `todo!()`, `unimplemented!()`, or mock data causes build failure.
* **Fail-Closed:** If a subsystem fails (e.g., pathfinding bug), the agent freezes or the engine panics. We do not fallback to "teleport."
* **Rust Authority:** 100% of the simulation logic is in Rust. No Python scripting for core rules.
* **Type Safety:** `AgentId` is a distinct type, not a `u64`. `Nutrient` is an enum, not a string. Realism is encoded in the type system.

---

## 12. What This System Enables (and What It Refuses)

### Enables

* **Generational Sociology:** Watch cultures rise and fall over centuries.
* **Economic Primordialism:** Value emerges from scarcity (calories/time), not fiat.
* **Ethical AI Research:** Agents that essentially *are* human allow checking alignment strategies in a high-stakes, isolated sandbox.

### Refuses

* **"Player" Centricity:** The universe does not care about you.
* **Narrative Causality:** Dramatic things do not happen "because it fits the story." They happen because vectors aligned.
* **Pay-to-Win:** Physics cannot be bribed.

---

*Verified by Antigravity / Markenz Architect.*
*Timestamp: 2026-01-11*
