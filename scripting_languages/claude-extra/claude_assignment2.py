# %% imports
import pandas as pd
import matplotlib.pyplot as plt

# %% Create DataFrame
df = pd.read_csv("../data/books_data.csv", parse_dates=["Date"])
df["Revenue"] = df["Price"] * df["Quantity"]
df

# %% a+c. Total revenue / average price per category
category_data = df.groupby("Category").agg(
    Total_Revenue=("Revenue", "sum"),
    Average_Price=("Price", "mean")
)

category_data

# %% b. Top 3 best selling books by quantity
df["Quantity"].nlargest(3)

# %% d. TODO! Determine the most popular book category among different age groups (you can define the age groups)
age_groups = [0, 12, 18, 40, 80, 100]
labels = ["children", "teen", "younger adult", "older adult", "elderly"]
df["Customer_AgeGroup"] = pd.cut(df["Customer_Age"], bins=age_groups, labels=labels, right=False)

df.groupby("Customer_AgeGroup")[""]

# %% e. Daily Revenue Trend
df.plot(x="Date", y="Revenue")
df[["Date", "Revenue"]]

# %% f. Find correlation between book price and quantity sold
ax = df[["Price", "Quantity"]].plot.bar()

# if a book is really expensive, then it's sales will be lower, because people don't want to buy it