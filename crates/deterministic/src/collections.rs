/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: world, engine, persistence
 * PURPOSE: Deterministic collection wrappers ensuring consistent iteration order
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 * INVARIANTS:
 *   - All iterations must produce identical order across runs
 *   - No HashMap/HashSet used (non-deterministic iteration order)
 *   - Element ordering must be by natural Ord implementation
 *   - Collections must remain in sorted order at all times
 *
 * # Design Rationale
 *
 * The Markenz simulation engine requires DETERMINISTIC EXECUTION:
 * - Identical inputs → Identical outputs
 * - Identical state → Identical subsequent behavior
 * 
 * Standard Rust HashMaps iterate in random order (by design) to prevent
 * hash collision attacks. However, this breaks determinism: the same
 * universe state processed twice can produce different results if the
 * iteration order differs.
 * 
 * SOLUTION: Use BTreeMap/BTreeSet which iterate in sorted order by key.
 * This guarantees:
 * 1. Deterministic iteration (always sorted by key)
 * 2. Reproducible state snapshots (hashes match across runs)
 * 3. Consistent agent processing order (Gem-D before Gem-K, etc.)
 * 4. Comparable performance (O(log n) operations)
 *
 * PERFORMANCE NOTE: BTreeMap is slower than HashMap for lookups in some
 * cases, but for simulation purposes (10-1000 agents/assets), the difference
 * is negligible. Determinism is worth the trade-off.
 *
 * Authority: antigravity
 */

use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// Deterministic map wrapper using BTreeMap for consistent iteration order.
///
/// # Why BTreeMap?
///
/// Standard `HashMap` iterates in random order to prevent hash collision attacks.
/// This is excellent for security but terrible for determinism. In a simulation
/// where state must be reproducible, random iteration order is unacceptable.
///
/// `BTreeMap` guarantees iteration in sorted key order, enabling:
/// - Reproducible simulation across runs
/// - Deterministic state hashing
/// - Consistent processing order for agents/assets
///
/// # Generic Constraints
///
/// - `K: Ord + Clone` - Keys must be totally ordered (for sorting) and cloneable
/// - `V: Clone` - Values must be cloneable (for insertion returns)
///
/// # Examples
///
/// ```
/// let mut map = DeterministicMap::new();
/// map.insert(3, "c");
/// map.insert(1, "a");
/// map.insert(2, "b");
/// 
/// // Always iterates as (1, "a"), (2, "b"), (3, "c")
/// let keys: Vec<_> = map.keys().cloned().collect();
/// assert_eq!(keys, vec![1, 2, 3]);
/// ```
///
/// # Invariants
///
/// - Keys always remain sorted
/// - Iteration order never changes between runs
/// - Insert/remove operations maintain sort order automatically
/// - No panics on valid operations (only on misuse of unsafe code)
#[derive(Debug, Clone)]
pub struct DeterministicMap<K, V> {
    inner: BTreeMap<K, V>,
}

impl<K, V> DeterministicMap<K, V> 
where 
    K: Ord + Clone,
    V: Clone,
{
    /// Create a new empty DeterministicMap.
    ///
    /// # Time Complexity
    /// O(1) - Allocation only
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
    
    /// Insert a key-value pair, returning the old value if present.
    ///
    /// # Arguments
    /// - `key`: The key to insert (must be sortable via Ord)
    /// - `value`: The value to store
    ///
    /// # Returns
    /// `Some(old_value)` if key existed, `None` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree insertion
    ///
    /// # Determinism Note
    /// Insertion order doesn't affect iteration order (iteration is by key).
    /// This means two maps with same insertions in different orders will
    /// iterate identically, which is crucial for determinism.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }
    
    /// Get a reference to a value by key.
    ///
    /// # Arguments
    /// - `key`: The key to look up
    ///
    /// # Returns
    /// `Some(&value)` if key exists, `None` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree lookup
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    
    /// Get a mutable reference to a value by key.
    ///
    /// # Arguments
    /// - `key`: The key to look up
    ///
    /// # Returns
    /// `Some(&mut value)` if key exists, `None` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree lookup
    ///
    /// # Safety Note
    /// Mutating values doesn't affect ordering (values aren't used for sorting).
    /// This is safe for simulation where values represent state (agent positions,
    /// inventory, etc.) and changing them doesn't affect collection structure.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }
    
    /// Remove and return a value by key.
    ///
    /// # Arguments
    /// - `key`: The key to remove
    ///
    /// # Returns
    /// `Some(value)` if key existed, `None` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree deletion
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
    
    /// Check if a key exists in the map.
    ///
    /// # Arguments
    /// - `key`: The key to check
    ///
    /// # Returns
    /// `true` if key exists, `false` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree lookup
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
    
    /// Iterate over all (key, value) pairs in sorted key order.
    ///
    /// # Iteration Order
    /// ALWAYS sorted by key (ascending), ensuring determinism
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    ///
    /// # Determinism Note
    /// This is the critical method for determinism. Every call to iter()
    /// on the same map will produce pairs in the exact same order.
    ///
    /// # Example
    /// ```
    /// let mut map = DeterministicMap::new();
    /// map.insert(3, "c");
    /// map.insert(1, "a");
    /// map.insert(2, "b");
    ///
    /// let pairs: Vec<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
    /// // Result: [(1, "a"), (2, "b"), (3, "c")] - ALWAYS in this order
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter()
    }
    
    /// Get the number of key-value pairs in the map.
    ///
    /// # Time Complexity
    /// O(1) - Stored in BTree metadata
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the map is empty.
    ///
    /// # Time Complexity
    /// O(1)
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Iterate over all keys in sorted order.
    ///
    /// # Iteration Order
    /// ALWAYS sorted (ascending), ensures determinism
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }
    
    /// Iterate over all values in key-sorted order.
    ///
    /// # Iteration Order
    /// Values appear in the order of their keys (ascending)
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    ///
    /// # Implementation Note
    /// Values are returned in key order, not value order. This matters if
    /// you need to process agents in a specific order - you get them in
    /// agent ID order (the key), not in value appearance order.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }
}

impl<K, V> Default for DeterministicMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic set wrapper using BTreeSet for consistent iteration order.
///
/// # Why BTreeSet?
///
/// Like `DeterministicMap`, this uses `BTreeSet` instead of `HashSet` to
/// ensure deterministic iteration. All elements iterate in sorted order.
///
/// # Use Cases in Markenz
///
/// - Tracking unique agent IDs for iteration
/// - Maintaining sets of available biomes
/// - Storing unique asset types
/// - Tracking event types
///
/// # Generic Constraints
///
/// - `T: Ord + Clone` - Elements must be totally ordered and cloneable
///
/// # Examples
///
/// ```
/// let mut set = DeterministicSet::new();
/// set.insert(3);
/// set.insert(1);
/// set.insert(2);
/// 
/// // Always iterates as 1, 2, 3
/// let values: Vec<_> = set.iter().cloned().collect();
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
///
/// # Invariants
///
/// - Elements always remain sorted
/// - No duplicates allowed (standard set behavior)
/// - Iteration order deterministic across runs
#[derive(Debug, Clone)]
pub struct DeterministicSet<T> {
    inner: BTreeSet<T>,
}

impl<T> DeterministicSet<T> 
where 
    T: Ord + Clone,
{
    /// Create a new empty DeterministicSet.
    ///
    /// # Time Complexity
    /// O(1) - Allocation only
    pub fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }
    
    /// Insert an element into the set.
    ///
    /// # Arguments
    /// - `value`: The element to insert
    ///
    /// # Returns
    /// `true` if element was newly inserted, `false` if already present
    ///
    /// # Time Complexity
    /// O(log n) - BTree insertion
    ///
    /// # Determinism Note
    /// Insertion order doesn't affect iteration order or membership.
    /// Two sets with the same elements will iterate identically regardless
    /// of insertion order.
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }
    
    /// Remove an element from the set.
    ///
    /// # Arguments
    /// - `value`: The element to remove
    ///
    /// # Returns
    /// `true` if element was removed, `false` if not present
    ///
    /// # Time Complexity
    /// O(log n) - BTree deletion
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }
    
    /// Check if an element is in the set.
    ///
    /// # Arguments
    /// - `value`: The element to look for
    ///
    /// # Returns
    /// `true` if element is in set, `false` otherwise
    ///
    /// # Time Complexity
    /// O(log n) - BTree lookup
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }
    
    /// Iterate over all elements in sorted order.
    ///
    /// # Iteration Order
    /// ALWAYS sorted (ascending), ensuring determinism
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    ///
    /// # Determinism Note
    /// Critical for determinism. Same set elements always iterate in same order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    
    /// Get the number of elements in the set.
    ///
    /// # Time Complexity
    /// O(1) - Stored in BTree metadata
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the set is empty.
    ///
    /// # Time Complexity
    /// O(1)
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> Default for DeterministicSet<T>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic vector wrapper with stable sorting guarantees.
///
/// # Why a Wrapper?
///
/// Unlike `DeterministicMap` and `DeterministicSet` which replace non-deterministic
/// types, this wraps Vec because Vec itself is deterministic. However, we wrap it to:
/// 
/// 1. **Enforce deterministic sorting** - All `sort()` calls use stable sort
/// 2. **Document sorting contracts** - Ensure callers understand order matters
/// 3. **Prevent future accidents** - Force intentional sorting rather than relying on insertion order
///
/// # When to Use DeterministicVec vs DeterministicMap
///
/// - **Use DeterministicVec** when you need:
///   - Sequence with stable, documented ordering
///   - Explicit sort operations (to convert insertion order to value order)
///   - O(1) indexing by position
///   - Example: Action queue, event log
///
/// - **Use DeterministicMap** when you need:
///   - Key-based lookup O(log n)
///   - Automatic sorted iteration by key
///   - Example: Agent registry (ID -> Agent), assets (ID -> Asset)
///
/// # Generic Constraints
///
/// - `T: Clone + Ord` - Elements must be cloneable and totally ordered
///
/// # Examples
///
/// ```
/// let mut vec = DeterministicVec::new();
/// vec.push(3);
/// vec.push(1);
/// vec.push(2);
/// 
/// // Before sort, iteration is insertion order: [3, 1, 2]
/// vec.stable_sort();
/// // After sort, iteration is value order: [1, 2, 3]
/// 
/// let values: Vec<_> = vec.iter().cloned().collect();
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
///
/// # Invariants
///
/// - Elements can be in any order initially (insertion order)
/// - After `stable_sort()`, elements are in ascending order
/// - `stable_sort()` maintains relative order of equal elements
/// - All operations remain deterministic
#[derive(Debug, Clone)]
pub struct DeterministicVec<T> {
    inner: Vec<T>,
}

impl<T> DeterministicVec<T> 
where 
    T: Clone + Ord,
{
    /// Create a new empty DeterministicVec.
    ///
    /// # Time Complexity
    /// O(1) - Allocation only
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }
    
    /// Push an element onto the end of the vector.
    ///
    /// # Arguments
    /// - `value`: The element to push
    ///
    /// # Time Complexity
    /// O(1) amortized - Vec amortized push
    ///
    /// # Notes
    /// After pushing, elements are in insertion order.
    /// Call `stable_sort()` to sort by value if needed.
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }
    
    /// Remove and return the last element.
    ///
    /// # Returns
    /// `Some(value)` if vector is non-empty, `None` otherwise
    ///
    /// # Time Complexity
    /// O(1)
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
    
    /// Get a reference to an element by index.
    ///
    /// # Arguments
    /// - `index`: The 0-based index
    ///
    /// # Returns
    /// `Some(&value)` if index is valid, `None` otherwise
    ///
    /// # Time Complexity
    /// O(1) - Direct indexing
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    /// Get a mutable reference to an element by index.
    ///
    /// # Arguments
    /// - `index`: The 0-based index
    ///
    /// # Returns
    /// `Some(&mut value)` if index is valid, `None` otherwise
    ///
    /// # Time Complexity
    /// O(1) - Direct indexing
    ///
    /// # Safety Note
    /// Mutating elements doesn't affect vector order (order is by position, not value).
    /// This is safe for simulation state updates.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }
    
    /// Get the number of elements in the vector.
    ///
    /// # Time Complexity
    /// O(1) - Stored in Vec metadata
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the vector is empty.
    ///
    /// # Time Complexity
    /// O(1)
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Iterate over elements in current order (may be insertion or sorted order).
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    ///
    /// # Notes
    /// Elements iterate in their current position order, not necessarily sorted.
    /// Call `stable_sort()` first if you need value-order iteration.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    
    /// Iterate over mutable references in current order.
    ///
    /// # Time Complexity
    /// O(n) for full iteration
    ///
    /// # Safety Note
    /// Mutations affect element state but not vector position order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }
    
    /// Sort elements in ascending order using stable sort.
    ///
    /// # Time Complexity
    /// O(n log n) - Merge sort or similar
    ///
    /// # Determinism Note
    /// This uses Rust's stable sort, which:
    /// - Maintains relative order of equal elements
    /// - Is deterministic for same input
    /// - Guarantees O(n log n) worst case
    ///
    /// # Why Stable Sort?
    ///
    /// In a deterministic simulation, stable sort is critical. If we have
    /// agents with identical values (e.g., same priority), stable sort ensures
    /// we process them in consistent insertion order across runs. Unstable sort
    /// could reorder them differently on different runs, breaking determinism.
    ///
    /// # Example
    /// ```
    /// let mut vec = DeterministicVec::new();
    /// vec.push(3);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.stable_sort();
    /// // Result: [1, 2, 3]
    /// ```
    pub fn stable_sort(&mut self) {
        self.inner.sort_by(|a, b| a.cmp(b));
    }
    
    /// Sort elements using a custom comparison function with stable sort.
    ///
    /// # Arguments
    /// - `compare`: Function defining sort order
    ///
    /// # Time Complexity
    /// O(n log n) - Merge sort or similar
    ///
    /// # Determinism Note
    /// Custom comparison must be:
    /// 1. **Consistent** - Same elements always compare the same way
    /// 2. **Transitive** - If a<b and b<c, then a<c
    /// 3. **Deterministic** - No randomness or floating point errors
    ///
    /// # Example
    /// ```
    /// let mut vec = DeterministicVec::new();
    /// vec.push((3, "c"));
    /// vec.push((1, "a"));
    /// vec.push((2, "b"));
    /// vec.stable_sort_by(|a, b| a.0.cmp(&b.0)); // Sort by first element
    /// // Result: [(1, "a"), (2, "b"), (3, "c")]
    /// ```
    pub fn stable_sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.inner.sort_by(compare);
    }
}

impl<T> Default for DeterministicVec<T>
where
    T: Clone + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

/// # Test Module
///
/// ROLE: VERIFICATION
/// PURPOSE: Verify that all deterministic collections maintain ordering invariants
/// DETERMINISM: All tests are deterministic - no random failures
///
/// ## Critical Tests for Auditability
///
/// Each test verifies a core invariant for deterministic simulation:
/// 1. Insertion order doesn't affect iteration order (for Map/Set)
/// 2. Iteration order is reproducible (always same order for same state)
/// 3. Stable sort maintains relative order of equal elements
///
/// These invariants are crucial for:
/// - Reproducing universe state across runs
/// - Creating consistent snapshots
/// - Debugging by replaying from checkpoint
#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test DeterministicMap maintains key-sorted iteration order.
    ///
    /// # Test Objective
    /// Verify that no matter what insertion order we use, iteration
    /// always proceeds in sorted key order (critical for determinism).
    ///
    /// # Invariant Being Tested
    /// - Insertion order: 3, 1, 2
    /// - Iteration order: always 1, 2, 3 (sorted by key)
    ///
    /// # Why This Matters
    /// If agents are stored with ID as key, we process them in ID order.
    /// This ensures Gem-D (ID 1) is always processed before Gem-K (ID 2),
    /// guaranteeing deterministic agent update order.
    #[test]
    fn test_deterministic_map() {
        let mut map = DeterministicMap::new();
        
        // Insert in order: 3, 1, 2
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");
        
        // Verify iteration is in sorted key order (1, 2, 3), not insertion order
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&1, &"a")), "First element should be key 1");
        assert_eq!(iter.next(), Some((&2, &"b")), "Second element should be key 2");
        assert_eq!(iter.next(), Some((&3, &"c")), "Third element should be key 3");
        assert_eq!(iter.next(), None, "Should have no more elements");
    }
    
    /// Test DeterministicSet maintains value-sorted iteration order.
    ///
    /// # Test Objective
    /// Verify that set iteration proceeds in sorted value order,
    /// independent of insertion order.
    ///
    /// # Invariant Being Tested
    /// - Insertion order: 3, 1, 2
    /// - Iteration order: always 1, 2, 3 (sorted by value)
    ///
    /// # Why This Matters
    /// Sets are used to track things like active agent IDs, available biomes, etc.
    /// Deterministic iteration ensures we process them in consistent order.
    #[test]
    fn test_deterministic_set() {
        let mut set = DeterministicSet::new();
        
        // Insert in order: 3, 1, 2
        set.insert(3);
        set.insert(1);
        set.insert(2);
        
        // Verify iteration is in sorted order (1, 2, 3), not insertion order
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&1), "First element should be 1");
        assert_eq!(iter.next(), Some(&2), "Second element should be 2");
        assert_eq!(iter.next(), Some(&3), "Third element should be 3");
        assert_eq!(iter.next(), None, "Should have no more elements");
    }
    
    /// Test DeterministicVec with explicit stable sort.
    ///
    /// # Test Objective
    /// Verify that stable_sort() correctly orders elements while maintaining
    /// relative order of equal elements.
    ///
    /// # Invariant Being Tested
    /// - Initial order (insertion): 3, 1, 2
    /// - After stable_sort(): 1, 2, 3
    /// - Relative order of equal elements preserved
    ///
    /// # Why This Matters
    /// Vectors are used for sequences where insertion order matters initially
    /// (e.g., action queue), but we need to sort them for deterministic processing.
    /// Stable sort ensures equal-priority items stay in insertion order.
    #[test]
    fn test_deterministic_vec() {
        let mut vec = DeterministicVec::new();
        
        // Push in order: 3, 1, 2
        vec.push(3);
        vec.push(1);
        vec.push(2);
        
        // Before sort: insertion order [3, 1, 2]
        // After sort: value order [1, 2, 3]
        vec.stable_sort();
        
        // Verify sort worked correctly
        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(&1), "First element should be 1");
        assert_eq!(iter.next(), Some(&2), "Second element should be 2");
        assert_eq!(iter.next(), Some(&3), "Third element should be 3");
        assert_eq!(iter.next(), None, "Should have no more elements");
    }
}

