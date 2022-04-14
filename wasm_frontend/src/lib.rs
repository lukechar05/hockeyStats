mod utils;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};


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
	
	let mut teamVec: Vec<Team> = processAs(results);
	let mut highGs = -1.0; 
	let mut winnerTeam = String::new();

	// let mut avgGoalsPerTeam = &teamVec.iter().map(|x| getAvgGoals(x)).collect::<Vec<f32>>();

	for team in teamVec { 

		let mut avgGoalsPerTeam = getAvgGoals(&team);

		if avgGoalsPerTeam > highGs { 
			winnerTeam = team.name; 
			highGs = avgGoalsPerTeam;
		}
	}
	println!("{} is the best team", winnerTeam);
	println!("{} with an average goals of", highGs);
}

pub fn getAvgGoals(team: &Team) -> f32 { 

	let mut totalGoals = 0.0;
	for i in 0..14 { 
		let mut goals = team.players[i].goals;
		totalGoals = totalGoals + goals;
	}
	return totalGoals / 15.0;
}


pub fn processAs(val: &JsValue) -> Vec<Team> { 

	match val.into_serde::<Vec<Team>>() { 

		Ok(mut v) => { 
			return v
		},
		Err(_) =>  { 
			return vec![]
		}
	}
}
