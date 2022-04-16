from math import radians
import re
from flask import Flask, render_template, request, abort
import json
import random, string

app = Flask(__name__)

@app.route("/")
def root_page():
	return render_template("base.html")

# Your code here

if __name__ == "__main__":
	app.run()



# List of all hockey teams 
hockeyTeams = ["meerkats", "elephants", "antelopes", "seals", "seagulls", "starfish", "deer", "rabbits", "squirrels", "flamingos", "salamanders", "cranes"]


@app.route('/teams', methods = ["GET"])
def teamsList(): 
	return json.dumps(hockeyTeams)


@app.route('/teams/<team_name>', methods = ["GET"])
def getPlayerStats(team_name):
	if team_name not in hockeyTeams:

		return json.dumps(), 404

	else: 
		random.seed(team_name)
		listOfPlayers = []
		
		# Defense
		for i in range(6): 
			playerName = ''.join(random.choice(string.ascii_lowercase) for x in range(random.randint(4, 8)))
			pos = "D"
			goals = random.randrange(0, 25)
			assists = random.randrange(0, 25)
			listOfPlayers.append({ 
				"name": playerName,
				"position": pos,
				"assists": assists,
				"goals": goals 

			})
		
		# Center
		for i in range(3):
			playerName = ''.join(random.choice(string.ascii_lowercase) for x in range(random.randint(4, 8)))
			pos = "C"
			goals = random.randrange(0, 25)
			assists = random.randrange(0, 25)
			listOfPlayers.append({ 
				"name": playerName,
				"position": pos,
				"assists": assists,
				"goals": goals 

			})

		# Wingers
		for i in range(6):
			playerName = ''.join(random.choice(string.ascii_lowercase) for x in range(random.randint(4, 8)))
			pos = ""
			if (i < 3):
				pos = "LW"
			else: 
				pos = "RW"
			goals = random.randrange(0, 25)
			assists = random.randrange(0, 25)
			listOfPlayers.append({ 
				"name": playerName,
				"position": pos,
				"assists": assists,
				"goals": goals 

			})	
		return json.dumps(listOfPlayers)



	







	