from PyQt5.QtWidgets import (
    QApplication,
    QMainWindow,
    QWidget,
    QVBoxLayout,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QListWidget,
    QPushButton,
    QDialog,
    QTextEdit,
    QCalendarWidget
)
from PyQt5.QtCore import Qt
import sys
import json
import os

from task_details_dialog import TaskDetailsDialog

class ToDoList(QMainWindow):
    data_file = "tasks.json"
    def __init__(self):
        super().__init__()
        self.setWindowTitle("To-Do List")
        self.setFixedSize(500, 500)

        self.task_dict = {}

        self.create_widgets()
        self.modify_widgets()
        self.add_widgets_to_layouts()
        self.create_connections()
        self.load_data()

    def create_widgets(self):
        self.title_label = QLabel("To-Do List")

        self.new_task_input = QLineEdit()
        self.add_task_button = QPushButton("Add Task")
        self.remove_task_button = QPushButton("Remove Task")
        self.clear_list_button = QPushButton("Clear List")

        self.todo_list = QListWidget()

    def modify_widgets(self):
        self.title_label.setAlignment(Qt.AlignCenter)

    def add_widgets_to_layouts(self):
        self.main_layout = QVBoxLayout()

        self.task_input_layout = QHBoxLayout()
        self.task_input_layout.addWidget(QLabel("New Task:"))
        self.task_input_layout.addWidget(self.new_task_input)

        self.main_layout.addWidget(self.title_label)
        self.main_layout.addSpacing(20)
        self.main_layout.addLayout(self.task_input_layout)
        self.main_layout.addWidget(self.add_task_button)
        self.main_layout.addSpacing(20)
        self.main_layout.addWidget(self.todo_list)
        self.main_layout.addSpacing(20)
        self.button_layout = QHBoxLayout()
        self.button_layout.addWidget(self.remove_task_button)
        self.button_layout.addWidget(self.clear_list_button)
        self.main_layout.addLayout(self.button_layout)

        widget = QWidget()
        widget.setLayout(self.main_layout)

        self.setCentralWidget(widget)

    def create_connections(self):
        self.add_task_button.clicked.connect(self.add_task)
        self.remove_task_button.clicked.connect(self.remove_task)
        self.clear_list_button.clicked.connect(self.clear_list)
        self.todo_list.itemDoubleClicked.connect(self.show_task_details)

    def add_task(self):
        new_task = self.new_task_input.text()
        if new_task:
            task_num = self.todo_list.count() + 1
            self.todo_list.addItem(f"{task_num}. {new_task}")
            self.new_task_input.setText("")
            self.task_dict[task_num] = {
                "task": new_task,
                "notes": "",
                "due_date": ""
            }
            self.save_data()

    def remove_task(self):
        for item in self.todo_list.selectedItems():
            del self.task_dict[int(item.text()[0])]
            self.todo_list.takeItem(self.todo_list.row(item))
        self.renumber_tasks()
        self.save_data()

    def clear_list(self):
        self.todo_list.clear()
        self.task_dict.clear()
        self.save_data()

    def renumber_tasks(self):
        for i in range(self.todo_list.count()):
            item_text = self.todo_list.item(i).text()
            new_text = f"{i+1}. {item_text[3:]}"
            self.todo_list.item(i).setText(new_text)
            task_num = int(item_text[0])
            self.task_dict[i+1] = self.task_dict.pop(task_num)

    def show_task_details(self, item):
        task_num = int(item.text()[0])
        task_details = self.task_dict[task_num]
        dialog = TaskDetailsDialog(task_details)
        result = dialog.exec_()
        if result == QDialog.Accepted:
            self.task_dict[task_num] = dialog.get_task_details()
            self.save_data()

    def load_data(self):
        if os.path.exists(ToDoList.data_file):
            with open(ToDoList.data_file, "r") as f:
                data = json.load(f)
                if data:
                    self.task_list.clear()
                    self.task_data_list.clear()
                    for task_num, task_data in enumerate(data, start=1):
                        if 'task' in task_data:
                            task_text = f"{task_num}. {task_data['task']}"
                            self.task_list.addItem(task_text)
                            self.task_data_list.append(task_data)


    def save_data(self):
        with open("data.json", "w") as f:
            json.dump(self.task_dict, f)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    todo_list = ToDoList()
    todo_list.show()
    sys.exit(app.exec_())
