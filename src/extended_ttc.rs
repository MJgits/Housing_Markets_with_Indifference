use std::collections::HashMap;
use crate::graph::{IndifferenceMarket, Allocation,ObjectAvailability};

impl IndifferenceMarket{

    fn execute_extended_ttc(&self) -> Allocation {
        let k = 1;
        let A1 = &self.agents;

        let w = Allocation{
            allocation: HashMap::new(),
        };
        w	
    }
}
// while agent is remaining:
//
// 	- initialise S and S' as satisfied and unsatisfied agents
// 	- partition S' into Sk disjoint subsets
// 	- if Sk* is not empty: (removal and update phase)
// 		- for each agent in Sk*:
// 			- give each agent their currently endowed house and remove both from G
// 		- for each remaining agent ensure preferences and priorities are updated
// 	- Else (Sk* is empty, improvement phase)
// 		- construct a new graph G as per rule A (point each unsatisfied agent to the highest priority member in the smallest k subset of S, then all agents in Sk point to highest priority object in Sk-1 (which should exist by construction))
// 		- for each cycle on G:
// 			- for each agent in the cycle
// 				- change endowment to the house they point to

