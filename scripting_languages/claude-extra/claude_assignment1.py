"""Claude gives me data analysis questions, and I solve them"""

# %% imports
import pandas as pd
import matplotlib.pyplot as plt

# %% create DataFrame and add Total_Sales
df = pd.read_csv("../data/sales_data.csv")
df["Total_Sales"] = df.Units_Sold * df.Unit_Price
df

# %% a+b. Calculate the total sales / best selling for each product
product_data = df.groupby("Product").agg({
    "Total_Sales": "sum",
    "Units_Sold": "sum"
})

total_sales = product_data.Total_Sales

best_selling = product_data.Units_Sold.idxmax()

# %% c. Find the region with the highest total sales.
region_total_sales = df.groupby("Region")["Total_Sales"].sum().idxmax()
region_total_sales

# %% d. Create a daily sales trend (total sales per day)
sales_trend = df.groupby("Date")["Total_Sales"].sum()
sales_trend.plot(x="Date", y="Total Sales")

# %% e. Identify the day with the highest total sales.
sales_trend.idxmax()
