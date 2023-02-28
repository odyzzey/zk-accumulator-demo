/* 
Traditionally, a contract is composed of a state and users interface with them via public methods that can transition this state.

In an environment where external, but approved, public-facing methods do not have access to the state during execution (read: write only), how do we safely transition the contract's state with only the outputs of these methods?

Proposed Solution: Require that contracts implement accumulator logic to add output substates.

We build state transition logic into an addition operation between the state and public method output types, meaning that method outputs propose increments to the contract's state that the state object's addition operand can choose to accept or reject. These increments, or substates, can be added together, or accumulated to reduce the numbe of operations on the contract state itself.

For this to work it should be true that types $P$ and $V$ are such that if $P_t$ represents the contract's state at time $t$, and $V = \left\{ V_1, V_2, .., V_n\right\}\nonumber$ is a set of $n$ valid increments to $P$, we can say that $P_t + V_1 + V_2 + .. + V_n = P_t + \sum\limits_{i=1}^n (V_n)$
*/

#![allow(dead_code)]

// Path: src/lib.rs
// ContractPoint
// Contract state  
pub struct ContractPoint {
    x: i32,
    y: i32,
    total: u32,
}

// PointVote
// A single vote
pub struct PointVote {
    x: i32,
    y: i32,
    weight: u32,
}

// PointVote addition
// Accumulation logic for PointVotes
use std::ops::Add;
impl Add for PointVote {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: (self.x + other.x),
            y: (self.y + other.y),
            weight: self.weight + other.weight,
        }
    }
}

// ContractPoint addition
// Accumulatioon logic for ContractPoint  
impl Add<PointVote> for ContractPoint {
    type Output = Self;
    fn add(self, other: PointVote) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            total: self.total + other.weight,
        }
    }
}

// implement ContractPoint view methods
// these methods are public-facing and can be called by any user
// the do have access to the contract's state
impl ContractPoint {
    // get the current x value
    pub fn get_x(&self) -> i32 {
        self.x
    }

    // get the current y value
    pub fn get_y(&self) -> i32 {
        self.y
    }

    // get the current total value
    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_average(&self) -> (u64, u64) {
        let x = self.x as f64;
        let y = self.y as f64;
        let total = self.total as f64;
        ((x / total) as u64, (y / total) as u64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //test PointVote accumulation
    #[test]
    fn test_point_vote_addition() {
        // PointVotes accumulated from the same votes should be equal, regardless of order or nesting
        // PointVote + PointVote
        let vote1 = PointVote { x: 1, y: 1, weight: 1 };
        let vote2 = PointVote { x: 2, y: 2, weight: 2 };
        let vote3 = PointVote { x: 3, y: 3, weight: 3 };

        // (vote1 + vote2) + vote3 should equal vote1 + (vote2 + vote3)
        let accum1 = vote1 + vote2;

        assert_eq!(accum1.weight, 3);
        assert_eq!(accum1.x, 3);
        assert_eq!(accum1.y, 3);

        let accum2 = accum1 + vote3;

        assert_eq!(accum2.weight, 6);
        assert_eq!(accum2.x, 6);
        assert_eq!(accum2.y, 6);
    }

    // test incrementing ContractPoint state by a PointVote 
    #[test]
    fn test_contract_point_addition() {
        // ContractPoint + PointVote
        let mut point = ContractPoint { x: 0, y: 0, total: 0 };
        let vote1 = PointVote { x: 1, y: 1, weight: 1 };
        let vote2 = PointVote { x: 2, y: 2, weight: 2 };
        let vote3 = PointVote { x: 3, y: 3, weight: 3 };

        let accum1 = vote1 + vote2; 

        point = point + accum1;

        assert_eq!(point.x, 3);
        assert_eq!(point.y, 3);
        assert_eq!(point.total, 3);
        assert_eq!(point.get_average(), (1, 1));
        
        point = point + vote3;

        assert_eq!(point.x, 6);
        assert_eq!(point.y, 6);
        assert_eq!(point.total, 6);
        assert_eq!(point.get_average(), (1, 1));
    }
}
