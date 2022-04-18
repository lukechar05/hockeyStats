mod utils;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};


// Player and Team structs identical to classes for Player and Team in javascript


#[derive(Serialize, Deserialize)]
pub struct Team { 
	name: String, 
	players: Vec<Player>
}

#[derive(Serialize, Deserialize)]
pub struct Player { 
	name: String, 
	position: String,
	goals: f32, 
	assists: i32,
}


#[wasm_bindgen]
pub fn wasm_avg_goals(results: &JsValue) {
	
	// Call processAs to turn results into a Vector of teams
	let mut teamVec: Vec<Team> = processAs(results);
	let mut highGs = -1.0; 
	let mut winnerTeam = String::new();

	// loop through all teams
	for team in teamVec { 

		// get each teams average goals
		let mut avgGoalsPerTeam = getAverage(&team);

		// if its higher than previous highest avg goal count set this value correspondingly
		if avgGoalsPerTeam > highGs { 
			winnerTeam = team.name; 
			highGs = avgGoalsPerTeam;
		}
	}

	// print to console
	web_sys::console::log_2(&"The best team is".into(), &JsValue::from_serde(&winnerTeam).unwrap());
	web_sys::console::log_2(&"with an average goals of".into(), &JsValue::from_serde(&highGs).unwrap());
}

// Function to input a team, loop through their players and get that teams average goals
pub fn getAverage(team: &Team) -> f32 { 

	let mut totalGoals = 0.0;
	for i in 0..15 { 
		let mut goals = team.players[i].goals;
		totalGoals = totalGoals + goals;
	}
	return totalGoals / 15.0;
}


//  Make some javascripty some crabby rusty 
pub fn processAs(val: &JsValue) -> Vec<Team> { 

	match val.into_serde::<Vec<Team>>() { 

		Ok(mut v) => { 
			return v
		},
		// In the case of an error return an empty vec
		Err(_) =>  { 
			return vec![]
		}
	}
}
