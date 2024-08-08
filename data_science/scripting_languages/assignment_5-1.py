from decimal import Decimal

from pydantic import BaseModel, Field


class ProductionWorker(BaseModel):
    pay: Decimal = Field(decimal_places=2, frozen=True)
    shift: int = Field(gt=0, lt=3, frozen=True)
    name: str = Field(frozen=True)
    id_num: int = Field(frozen=True)

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