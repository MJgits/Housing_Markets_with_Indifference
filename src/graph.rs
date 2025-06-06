/*
This module should hold the graph implementation for any indifference market algo


We need general graph structure: likely an array of sets.

Ability to:
add and remove nodes 
add and remove edges

look for sources and sinks


Need some reverse look up for object Id : owner 
*/

use std::collections::HashMap;

use crate::agent::Agent;
pub struct IndifferenceMarket {
    pub agents: Vec<Agent>
}


// TODO need to figure out whether all this stuff relies on i32 ids or actual object structures (a copy of the object)
impl IndifferenceMarket {
    pub fn is_valid_market(&self) -> bool {
        todo!()
    }

    pub fn allocate_object_to_agent(&self, o:i32, a:i32) {
        todo!()
    }

    pub fn remove_agent_and_allocated_object(&self, a:i32, availabilities: &ObjectAvailability) {
        todo!()
    }

}


// Should eventually progress this to having set and get methods instead of direct access to fields
pub struct ObjectAvailability {
    pub availability: HashMap<i32, bool>
}

impl ObjectAvailability {

    pub fn add_object(&self, o: i32) {
        // should be an add or insert situation
        todo!()
    }

    pub fn set_availability (&self, o: i32, a: bool) {
        todo!()
    }
    pub fn get_availability() {
        todo!()
    }
}


pub struct Allocation {
    pub allocation: HashMap<i32,i32>
}

