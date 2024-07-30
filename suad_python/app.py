import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

from flask import Flask, render_template, request, redirect, url_for, send_file
import io

app = Flask(__name__)

DATA_FILE = 'static/game.csv'

def get_data():
    try:
        df = pd.read_csv(DATA_FILE, index_col=0)
    except FileNotFoundError:
        df = pd.DataFrame(columns=['Collin', 'Brennan', 'Ethan', 'Katelyn', 'Mom', 'Dad'])
    return df

@app.route('/')
def game_tracker():
    df = get_data()
    total_scores = df.sum()
    return render_template('game_tracker.html', 
                           game=df.values.tolist(),
                           players=df.columns.tolist(),
                           total_scores=total_scores.tolist())

@app.route('/export-csv')
def export_csv():
    df = get_data()
    data = df.to_csv()
    return data

@app.route('/add-round', methods=['POST'])
def add_round():
    scores = request.form.getlist('scores')
    df = get_data()
    new_row = pd.DataFrame([scores], columns=df.columns)
    df = pd.concat([df, new_row], ignore_index=True)
    df.to_csv(DATA_FILE)
    return redirect(url_for('game_tracker'))

@app.route('/game-graph')
def generate_graph():
    df = get_data()
    df.iloc[0] = 0

    fig, ax = plt.subplots(figsize=(9, 12))
    df.cumsum().plot(ax=ax)
    
    ax.set_xlabel('Round')
    ax.set_ylabel('Points')
    ax.legend()

    num_rounds = len(df)
    ax.set_xticks(range(0, num_rounds, 1))
    ax.set_xticklabels(range(0, num_rounds, 1))
    
    img = io.BytesIO()
    fig.savefig(img, format='jpg')
    img.seek(0)
    
    return send_file(img, mimetype='image/jpeg')

if __name__ == '__main__':
    app.run(debug=True)
