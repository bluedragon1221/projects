from pydantic import BaseModel

class ProductionWorker(BaseModel):
    name: str
    id_num: int
    shift: int
    pay: float

def print_info(employee: ProductionWorker):
    print("Production Worker Information")
    print("Name:", employee.name)
    print("ID number:", employee.id_num)
    print("Shift:", employee.shift)
    print("Hourly Pay Rate:", f"${employee.pay:,.2f}")

def main():
    name = input("Enter your name: ")
    id_num = input("Enter your ID number: ")
    shift = input("Enter the shift number: ")
    pay = input("Enter your hourly rate: ")

    production_worker = ProductionWorker(name=name, id_num=id_num, shift=shift, pay=pay)
    print_info(production_worker)

if __name__ == "__main__":
    main()