/*
Define agent struct here

should include a nice way to access preferences high to low with indifference and priority order

also needs to have labels and maybe group status depending on algo



agents probably look different depending on algorithm, so agent should potentially be a trait and then we make impls for each type of agent for important functions so they can be called together
*/

use std::collections::{hash_set, HashMap, HashSet};
use crate::graph::ObjectAvailability;




pub struct Agent {
    id: i32,
    preferences: Vec<HashSet<i32>>,
    endowment_id: i32
}


struct Object {
    id: i32,
    owner_id: i32,
}



// need some O(1) check of object availability or some top(A) function
impl Agent {


    // This function should 
    pub fn top_available_obj(&self, available_items: &ObjectAvailability) -> HashSet<i32> {

        let mut top_available: HashSet<i32> = HashSet::new();

        let prefs = &self.preferences;

        for equivalence_class in prefs {
            if top_available.is_empty() {
                for obj in equivalence_class {
                    if matches!(available_items.availability.get(&obj), Some(true)) {
                        top_available.insert(*obj);
                    }
                }
            }
        }
        top_available
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn top_available_test() {
        let equiv_class1:HashSet<i32> = [0,1].into_iter().collect();
        let equiv_class2:HashSet<i32> = [2,3].into_iter().collect();
        let equiv_class3:HashSet<i32> = [4].into_iter().collect();
        
        let agent1 = Agent{
            id: 0,
            preferences: vec![equiv_class1,equiv_class2,equiv_class3],
            endowment_id: 0,
        };

        let avails1 = ObjectAvailability{
            availability: [(0, false), (1,true), (2,false), (3, true), (4, true)].into_iter().collect()
        };

        let top_avail = agent1.top_available_obj(&avails1);
        
        let correct_answer:HashSet<i32> = [1].into_iter().collect();
        assert_eq!(top_avail,correct_answer);
    }
}



