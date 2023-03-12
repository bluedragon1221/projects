import sys
import json
from PyQt5.QtWidgets import QApplication, QWidget, QListWidget, QVBoxLayout, QLabel


class DisplayCards(QWidget):
    def __init__(self):
        super().__init__()

        # Load data from file
        with open('pykemon_cards.json', 'r') as f:
            self.data = json.load(f)

        # Create list widget
        self.card_list = QListWidget()
        self.card_list.itemClicked.connect(self.show_card_info)

        # Add cards to list widget
        for card in self.data:
            self.card_list.addItem(card['name'])

        # Create label for card info
        self.card_info_label = QLabel()

        # Create layout and add widgets
        layout = QVBoxLayout()
        layout.addWidget(self.card_list)
        self.setLayout(layout)

    def show_card_info(self, item):
        # Get card info from data
        for card in self.data:
            if card['name'] == item.text():
                card_info = json.dumps(card, indent=4)
                break

        # Update card info label
        self.card_info_label.setText(card_info)

        def generateCardText(self, card):
            text = f"<b>Name:</b> {card['name']}<br>"
            text += f"<b>Type:</b> {card['type']}<br>"
            text += f"<b>HP:</b> {card['hp']}<br>"
            text += "<b>Moves:</b><br>"
            for move in card['moves']:
                text += f"- {move['name']}: {move['damage']}<br>"
            return text

if __name__ == '__main__':
    app = QApplication(sys.argv)
    window = DisplayCards()
    window.show()
    sys.exit(app.exec_())
