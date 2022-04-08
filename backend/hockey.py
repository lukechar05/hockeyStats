from flask import Flask, render_template, request, abort

app = Flask(__name__)

@app.route("/")
def root_page():
	return render_template("base.html")

# Your code here

if __name__ == "__main__":
	app.run()
