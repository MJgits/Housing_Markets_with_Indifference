
use std::collections::HashMap;
use crate::graph::{IndifferenceMarket, Allocation,ObjectAvailability};


impl IndifferenceMarket {
    fn execute_ttc_arb_tie_break(&self) -> Allocation {


        // initialise all objects available
            // can this happen during running through preferences like and or insert i32, true?
        let mut available_items = ObjectAvailability {
            availability: HashMap::new()
        };

        for agent in &self.agents {
            available_items.add_object(agent.endowment_id.clone());
        }
        // initialise empty allocation
        // until no agent remains:
        //     agent i

        //     access their top available preference with arbitrary tie breaks

        //     find owner of available preference

        //     repeat until loop. FIGURE OUT THIS PART

        //     hand out objects within loop

        //      repeat with new agent
        
        Allocation {
            allocation: [(1,1)].into_iter().collect()
        }
    }

}


