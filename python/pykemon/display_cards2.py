import sys
import json
from PyQt5.QtWidgets import (
    QApplication,
    QWidget,
    QListWidget,
    QVBoxLayout,
    QDialog,
    QPlainTextEdit,
)


class DisplayCards(QWidget):
    def __init__(self):
        super().__init__()

        # Load data from file
        with open("pykemon_cards.json", "r") as f:
            self.data = json.load(f)

        # Create list widget
        self.card_list = QListWidget()
        self.card_list.itemClicked.connect(self.show_card_info)

        # Add cards to list widget
        for card in self.data:
            self.card_list.addItem(card["name"])

        # Create layout and add widgets
        layout = QVBoxLayout()
        layout.addWidget(self.card_list)
        self.setLayout(layout)
        self.setWindowTitle("Pykemon v0.1")


    def show_card_info(self, item):
        # Get card info from data
        for card in self.data:
            if card["name"] == item.text():
                card_info = json.dumps(card, indent=4)
                break

        # Display card info in a dialog box
        dialog = QDialog(self)
        dialog.setWindowTitle(item.text())
        dialog_layout = QVBoxLayout()

        text_edit = QPlainTextEdit()
        text_edit.setPlainText(card_info)
        text_edit.setReadOnly(True)

        dialog_layout.addWidget(text_edit)
        dialog.setLayout(dialog_layout)
        dialog.exec_()

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = DisplayCards()
    window.show()
    sys.exit(app.exec_())
