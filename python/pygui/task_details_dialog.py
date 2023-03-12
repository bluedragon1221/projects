from PyQt5.QtWidgets import (
    QDialog,
    QVBoxLayout,
    QHBoxLayout,
    QLabel,
    QTextEdit,
    QPushButton,
    QCalendarWidget
)
from PyQt5.QtCore import Qt

class TaskDetailsDialog(QDialog):
    def __init__(self, task_details):
        super().__init__()

        self.task_details = task_details

        self.create_widgets()
        self.modify_widgets()
        self.add_widgets_to_layouts()
        self.create_connections()

        self.setWindowTitle("Task Details")
        self.setFixedSize(400, 300)

    def create_widgets(self):
        self.task_label = QLabel(self.task_details["task"])
        self.notes_label = QLabel("Notes:")
        self.notes_input = QTextEdit(self.task_details["notes"])
        self.due_date_label = QLabel("Due Date:")
        self.due_date_input = QCalendarWidget()
        self.due_date_input.setSelectedDate(self.task_details["due_date"])
        self.cancel_button = QPushButton("Cancel")
        self.save_button = QPushButton("Save")

    def modify_widgets(self):
        self.task_label.setAlignment(Qt.AlignCenter)

    def add_widgets_to_layouts(self):
        self.main_layout = QVBoxLayout()

        self.task_layout = QVBoxLayout()
        self.task_layout.addWidget(QLabel("Task:"))
        self.task_layout.addWidget(self.task_label)

        self.notes_layout = QVBoxLayout()
        self.notes_layout.addWidget(self.notes_label)
        self.notes_layout.addWidget(self.notes_input)

        self.due_date_layout = QVBoxLayout()
        self.due_date_layout.addWidget(self.due_date_label)
        self.due_date_layout.addWidget(self.due_date_input)

        self.button_layout = QHBoxLayout()
        self.button_layout.addWidget(self.cancel_button)
        self.button_layout.addWidget(self.save_button)

        self.main_layout.addLayout(self.task_layout)
        self.main_layout.addSpacing(20)
        self.main_layout.addLayout(self.notes_layout)
        self.main_layout.addSpacing(20)
        self.main_layout.addLayout(self.due_date_layout)
        self.main_layout.addSpacing(20)
        self.main_layout.addLayout(self.button_layout)

        self.setLayout(self.main_layout)

    def create_connections(self):
        self.cancel_button.clicked.connect(self.reject)
        self.save_button.clicked.connect(self.accept)

    def get_task_details(self):
        self.task_details["notes"] = self.notes_input.toPlainText()
        self.task_details["due_date"] = self.due_date_input.selectedDate().toString(Qt.ISODate)
        return self.task_details
