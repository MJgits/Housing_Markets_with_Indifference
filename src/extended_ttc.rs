use std::{collections::{HashMap, HashSet}, os::unix::net::UnixStream};
use crate::graph::{AgentId, Allocation, IndifferenceMarket, ObjectAvailability};

impl IndifferenceMarket{

    pub fn execute_extended_ttc(&mut self)  {
        let mut k = 1;
        // is there a way to do this well without cloning?
        // work with agents clone for the rest of the algo so we dont eject agents from the struct itself
        let agents_clone = self.agents.clone();
        
        
        
        
        // while agent is remaining:
        while !agents_clone.is_empty() {
            
            // 	- initialise S and S' as satisfied and unsatisfied agents
            let mut satisfied_set: HashSet<AgentId> = HashSet::new();
            let mut unsatisfied_set: HashSet<AgentId> = HashSet::new();


            for agent in &agents_clone {
                if agent.is_satisfied(&self.object_availability) {
                    satisfied_set.insert(agent.id);
                } else {
                    unsatisfied_set.insert(agent.id);
                }
            };

            // we will insert S1..St, S* into satisfied_partition
            let mut satisfied_partition: Vec<Vec<i32>> = Vec::new();
            // partitioning satisfied agents by removing them from the bigger satisfied set and placing into satisfied_partition
            // be mindful of the total order system as well
            while !satisfied_set.is_empty() {

            }


            
            


        
        
        }
        

        // 



        
        


    }
}

//

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

