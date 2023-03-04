#![no_std]

#[derive(Debug, Clone, Copy)]
pub struct ContractPoint {
    x: i32,
    y: i32,
    total: u32,
}

// PointVote
pub struct PointVote {
    x: i32,
    y: i32,
    weight: u32,
}

//View methods for PointVote
impl PointVote {

    pub fn new(x: i32, y: i32, weight: u32) -> Self {
        Self { x, y, weight }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_weight(&self) -> u32 {
        self.weight
    }

    pub fn add(self, other: Self) -> Self{
        Self {
            x: (self.x + other.x),
            y: (self.y + other.y),
            weight: self.weight + other.weight,
        }
    }
}

// The contract's state
impl ContractPoint {

    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            total: 0,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_average(&self) -> (u64, u64) {
        let x = self.x as f64;
        let y = self.y as f64;
        let total = self.total as f64;
        ((x / total) as u64, (y / total) as u64)
    }

    pub fn add(self, other: PointVote) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            total: self.total + other.weight,
        }
    }
}


#[cfg(test)]
mod tests {
    /*
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
    */
}
