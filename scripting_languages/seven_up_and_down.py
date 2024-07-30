# %%
# imports
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

# %%
# create the DataFrame
df = pd.read_csv("data/seven_up_and_down.csv")
df.index.name = "Round"
df
# %%
# get total score per round, and plot
cs = df.cumsum()
cs.plot()

# %%
# generate stats per person
player_stats = pd.DataFrame({
    "Total_Score": df.sum(),
    "Average_Gain": df.mean(),
    "Biggest_Gain": df.max(),
    "Biggest_Drop": df.min(),
    "Highest_Score": cs.max(),
    "Lowest_Score": cs.min(),
    "Times_Increased": (df > 0).sum(),
    "Times_Decreased": (df < 0).sum()
})
player_stats

# %%
# generate stats about the game
# [TODO) Add more game stats
game_stats = pd.Series({
    "Winner": player_stats["Total_Score"].idxmax(),
    "Looser": player_stats["Total_Score"].idxmin(),
})
game_stats
