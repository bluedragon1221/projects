employee_records = [
    
]

employee_records.sort(key=lambda x: x[2])

young_employees = [tuple(person) for *person, age, _ in employee_records if age < 30]

print(young_employees)
