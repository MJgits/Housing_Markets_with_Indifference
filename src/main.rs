/* should just have test executions here 

Using agents and preferences from Xiong et al.
*/
use std::collections::HashSet;

use ttc_indifference::{agent::Agent, graph::IndifferenceMarket, preferences};

fn main() {
    println!("Hello, world!");
    
    let mut xiong_market = IndifferenceMarket::new(xiong_agents());

    xiong_market.execute_extended_ttc();
    

}

fn xiong_agents()-> Vec<Agent> {
    let mut agent1 = Agent{
        id:1,
        preferences: preferences!([1]),
        endowment_id: 1,
    };
    let mut agent2 = Agent{
        id:2 ,
        preferences: preferences!([1],[4,5]),
        endowment_id: 2,
    };
    let mut agent3 = Agent{
        id:3 ,
        preferences: preferences!([6]),
        endowment_id: 3,
    };
    let mut agent4 = Agent{
        id:4 ,
        preferences:preferences!([3,4,5,6]),
        endowment_id: 4,
    };
    let mut agent5 = Agent{
        id:5 ,
        preferences: preferences!([4,5]),
        endowment_id: 5,
    };
    let mut agent6 = Agent{
        id:6 ,
        preferences: preferences!([5]),
        endowment_id: 6,
    };

    vec![agent1,agent2,agent3,agent4,agent5,agent6]

}

    
