/**
 * ROLE: BOUNDARY
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * EXECUTED VIA: windsurf
 * USED BY: server
 * PURPOSE: Deterministic math functions
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

use std::f32;

/// Deterministic math utilities
pub struct DeterministicMath;

impl DeterministicMath {
    /// Deterministic sine approximation
    pub fn sin(x: f32) -> f32 {
        // Use a simple polynomial approximation for determinism
        // This avoids potential differences in libm implementations
        let x = x % (2.0 * f32::consts::PI);
        if x > f32::consts::PI {
            return -Self::sin(x - f32::consts::PI);
        }
        
        // Taylor series approximation (5 terms for good accuracy)
        let x2 = x * x;
        let x3 = x2 * x;
        let x5 = x3 * x2;
        let x7 = x5 * x2;
        let x9 = x7 * x2;
        
        x - (x3 / 6.0) + (x5 / 120.0) - (x7 / 5040.0) + (x9 / 362880.0)
    }
    
    /// Deterministic cosine approximation
    pub fn cos(x: f32) -> f32 {
        Self::sin(x + f32::consts::PI / 2.0)
    }
    
    /// Deterministic square root with fixed iteration count
    pub fn sqrt(x: f32) -> f32 {
        if x < 0.0 {
            return f32::NAN;
        }
        if x == 0.0 {
            return 0.0;
        }
        
        // Newton's method with fixed iterations for determinism
        let mut guess = x / 2.0;
        for _ in 0..10 {
            guess = (guess + x / guess) / 2.0;
        }
        guess
    }
    
    /// Deterministic absolute value
    pub fn abs(x: f32) -> f32 {
        if x < 0.0 { -x } else { x }
    }
    
    /// Deterministic minimum
    pub fn min(a: f32, b: f32) -> f32 {
        if a < b { a } else { b }
    }
    
    /// Deterministic maximum
    pub fn max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }
    
    /// Deterministic clamp
    pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
        Self::min(Self::max(x, min), max)
    }
    
    /// Deterministic linear interpolation
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_math() {
        // Test that results are consistent
        let result1 = DeterministicMath::sin(1.0);
        let result2 = DeterministicMath::sin(1.0);
        assert_eq!(result1, result2);
        
        // Test basic properties
        assert!(DeterministicMath::abs(-5.0) == 5.0);
        assert!(DeterministicMath::min(3.0, 7.0) == 3.0);
        assert!(DeterministicMath::max(3.0, 7.0) == 7.0);
        assert!(DeterministicMath::clamp(5.0, 0.0, 10.0) == 5.0);
        assert!(DeterministicMath::clamp(-5.0, 0.0, 10.0) == 0.0);
        assert!(DeterministicMath::clamp(15.0, 0.0, 10.0) == 10.0);
    }
}

