/*
This module should hold the graph implementation for any indifference market algo


We need general graph structure: likely an array of sets.

Ability to:
add and remove nodes 
add and remove edges

look for sources and sinks


Need some reverse look up for object Id : owner 
*/

use std::collections::{HashMap, HashSet};
use crate::agent::Agent;

pub type AgentId = i32;
pub type ObjectId = i32;


// should be an easy way to get preferences 
#[macro_export]
macro_rules! preferences {
    [ $( [ $( $x:literal ),* ] ),* ] => {{
        let mut prefs = Vec::new();
        $(
            let mut set = std::collections::HashSet::new();
            $(
                set.insert($x);
            )*
            prefs.push(set);
        )*
        prefs
    }};
}




// This should probably contain the allocations and object availabilities?
pub struct IndifferenceMarket {
    pub agents: Vec<Agent>,
    pub object_availability: ObjectAvailability,
    pub allocation: HashMap<AgentId,ObjectId>
}


// TODO need to figure out whether all this stuff relies on i32 ids or actual object structures (a copy of the object)
impl IndifferenceMarket {

    pub fn new(agent_vector: Vec<Agent>) -> Self {

        let mut market = IndifferenceMarket {
            agents: agent_vector,
            object_availability: ObjectAvailability { availability: HashMap::new() } ,
            allocation: HashMap::new(),
        };


        // initialise agents with their own objects and object availabilities
        for agent in &market.agents {

            market.object_availability.add_object(agent.endowment_id);
            // unwraps necessary to convert usize to ints
            market.allocation.insert(agent.id, agent.endowment_id);
        }
        market
    }
    pub fn is_valid_market(&self) -> bool {
        todo!()
    }

    pub fn allocate_object_to_agent(&mut self, o:ObjectId, a:AgentId) {
        self.allocation.entry(a).insert_entry(o);
    }

    pub fn remove_agent_and_allocated_object(&self, a:i32, availabilities: &ObjectAvailability) {
        todo!()
    }

}


// Should eventually progress this to having set and get methods instead of direct access to fields

// serves as a reverse look up for 
pub struct ObjectAvailability {
    pub availability: HashMap<ObjectId, bool>
    
}

impl ObjectAvailability {
    // is this even a necessary method?
    pub fn add_object(&mut self, o: ObjectId) {
        // should be an add or insert situation
        self.availability.insert(o, true);
    }

    pub fn set_availability (&mut self, o: ObjectId, a: AgentId) {
        todo!()
    }

    pub fn get_availability(&self, o:ObjectId) -> bool {
        *self.availability.get(&o).unwrap()
    }
}


pub struct Allocation {
    pub allocation: HashMap<AgentId,ObjectId>
}

impl Allocation {
    pub fn new(&self, length:usize) {

    }

}

