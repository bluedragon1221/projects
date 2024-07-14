def calculate_revenue(cups_sold, price_per_cup):
    return cups_sold * price_per_cup

coffee_menu = {
    "Espresso": 3.0,
    "Latte": 4.0,
    "Cappuccino": 5.0
}

def calculate_total_revenue(sales_data):
    total_revenue = 0
    for item, amount in sales_data.items():
        item_price = coffee_menu.get(item)
        total_revenue += calculate_revenue(amount, item_price)

    return total_revenue

sales_data = {
    "Espresso": 20,
    "Latte": 40,
    "Cappuccino": 32
}

total_revenue = calculate_total_revenue(sales_data)
print(f"Total Revenue Generated: $ {total_revenue}")
