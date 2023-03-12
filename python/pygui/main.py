from PyQt5.QtWidgets import QApplication
import sys
from todolist import ToDoList

if __name__ == "__main__":
    app = QApplication(sys.argv)
    todo_list = ToDoList()
    todo_list.show()
    sys.exit(app.exec_())
