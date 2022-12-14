
let teams = [];

class hockeyTeam {
    name = "";
    players = [];
    constructor(name, players)
    {
        this.name = name;
        this.players = players;
    }
}

class Player { 
    name = "";
	position = "";
	goals = 0;
	assists = 0;

    constructor(name, position, goals, assists) { 
        this.name = name;
        this.position = position;
        this.goals = goals; 
        this.assists = assists;
    }

}

export function process_all_teams(func) {


    // Get all the teams
    fetch("/teams")
    .then((response) => {
        return response.json();
    })
    .then((result) => {

        // GOATED LINE OF CODE. MAP 15 players to each team and get that whole biddie in a json 
        Promise.all(result.map(team => fetch("/teams/" + team).then(response => response.json())))
        .then((filledTeams) => { 
            let playersArray = []; 

            for (var x = 0; x < result.length; x++) {
                for (var i = 0; i < 15; i++) { 

                    // Create player object and add it 15 times
                    let newPlayer = new Player(filledTeams[x][i].name, filledTeams[x][i].position, filledTeams[x][i].goals, filledTeams[x][i].assists);
                    playersArray.push(newPlayer);
                    
                }
                //  Now you have 15 players which can make a full team 
                let newTeam = new hockeyTeam(result[x], playersArray);
                teams.push(newTeam);
                playersArray = [];
                
            }  
            return teams;
        })
        .then((endResult) => { 
           func(endResult);
        })
    })
    
    //  Catch any errors in this long block of code
    .catch(() => {
        console.log("Error fetching teams");
    });
}

//  Function to calculate the average goals in java script 
export function js_avg_goals(results) {


    let highGs = -1;
    let winner = "";

    // Loop throught all the teams
    for (var i = 0; i < results.length; i++) {
        
		let totalGoals = 0
        // Go through each player on the time and find this teams avg goals
		for (var j = 0; j < 15; j++)
		{   
			totalGoals = totalGoals + results[i].players[j].goals;
		}
		let avgGoals = totalGoals / 15.0;

        // If its higher than previous highestGoals set the 2 global variables as necessary 
        if(avgGoals > highGs)
        {
            highGs = avgGoals;
            winner = results[i].name;
        }
    }
    console.log(" The team with the highest average goals is " + winner + "and they have an average of " + highGs)
}