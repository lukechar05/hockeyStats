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

	for team in teamVec { 

		let mut avgGoalsPerTeam = getAverage(&team);

		if avgGoalsPerTeam > highGs { 
			winnerTeam = team.name; 
			highGs = avgGoalsPerTeam;
		}
	}
	web_sys::console::log_2(&"The best team is".into(), &JsValue::from_serde(&winnerTeam).unwrap());
	web_sys::console::log_2(&"with an average goals of".into(), &JsValue::from_serde(&highGs).unwrap());
}

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
		Err(_) =>  { 
			return vec![]
		}
	}
}
