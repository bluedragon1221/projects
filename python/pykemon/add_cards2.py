import sys
import json
from PyQt5.QtWidgets import (
    QApplication, QWidget, QLabel, QLineEdit, QComboBox, QSpinBox,
    QPushButton, QVBoxLayout, QHBoxLayout, QDialog, QFormLayout
)

import typing

class AddCardDialog(QDialog):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Add Card")
        self.nameLabel = QLabel("Name:")
        self.nameEdit = QLineEdit()
        self.typeLabel = QLabel("Type:")
        self.typeCombo = QComboBox()
        self.typeCombo.addItems(["Normal", "Fire", "Water", "Grass", "Electric", "Dark"])
        self.hpLabel = QLabel("HP:")
        self.hpSpin = QSpinBox()
        self.hpSpin.setRange(0, 500)
        self.movesLabel = QLabel("Moves:")
        self.moveEdits = []
        self.damageSpins = []
        self.addMoveButton = QPushButton("Add Move")
        self.addButton = QPushButton("Add")
        self.cancelButton = QPushButton("Cancel")

        # Create layout
        formLayout = QVBoxLayout()
        formLayout.addWidget(self.nameLabel)
        formLayout.addWidget(self.nameEdit)
        formLayout.addWidget(self.typeLabel)
        formLayout.addWidget(self.typeCombo)
        formLayout.addWidget(self.hpLabel)
        formLayout.addWidget(self.hpSpin)

        movesLayout = QVBoxLayout()
        movesLayout.addWidget(self.movesLabel)
        self.addMoveButton.clicked.connect(self.addMove)
        movesLayout.addWidget(self.addMoveButton)
        formLayout.addLayout(movesLayout)

        buttonLayout = QHBoxLayout()
        buttonLayout.addWidget(self.addButton)
        buttonLayout.addWidget(self.cancelButton)

        mainLayout = QVBoxLayout()
        mainLayout.addLayout(formLayout)
        mainLayout.addLayout(buttonLayout)

        self.setLayout(mainLayout)

        # Connect button signals to slots
        self.addButton.clicked.connect(self.saveToFile)
        self.cancelButton.clicked.connect(self.reject)

    def saveToFile(self) -> None:
        card_name    = self.nameEdit.text()
        card_type    = self.typeCombo.currentText()
        card_hp      = self.hpSpin.value()
        moves_list: list[str] = [i.text() for i in self.moveEdits]
        damages_list: list[int] = [i.value() for i in self.damageSpins]

        # Create dictionary for card info
        cardInfo: dict[str, typing.Any] = {
            'name': card_name,
            'type': card_type,
            'hp': card_hp,
            'moves': []  # List[Dict[str, Any]]
        }

        # Add move info to dictionary
        for move, damage in zip(moves_list, damages_list):
            moveInfo: dict[str, typing.Any] = {
                'name': move,
                'damage': damage
            }
            cardInfo['moves'].append(moveInfo)

        # Append dictionary to JSON file
        with open("pykemon_cards.json", 'r') as f:
            cards: list[dict[str, typing.Any]] = json.load(f)
            cards.append(cardInfo)

        with open("pykemon_cards.json", 'w') as f:
            json.dump(cards, f)

        self.reject()


    def addMove(self):
        # Create dialog to get move info
        dialog = QDialog(self)
        dialog.setWindowTitle("Add Move")
        nameLabel = QLabel("Name:")
        nameEdit = QLineEdit()
        damageLabel = QLabel("Damage:")
        damageSpin = QSpinBox()
        damageSpin.setRange(0, 500)
        addButton = QPushButton("Add")
        cancelButton = QPushButton("Cancel")
        formLayout = QFormLayout()
        formLayout.addRow(nameLabel, nameEdit)
        formLayout.addRow(damageLabel, damageSpin)
        buttonLayout = QHBoxLayout()
        buttonLayout.addWidget(addButton)
        buttonLayout.addWidget(cancelButton)
        mainLayout = QVBoxLayout()
        mainLayout.addLayout(formLayout)
        mainLayout.addLayout(buttonLayout)
        dialog.setLayout(mainLayout)

        # Connect button signals to slots
        addButton.clicked.connect(dialog.accept)
        cancelButton.clicked.connect(dialog.reject)

        # Show dialog and wait for user input
        if dialog.exec_() == QDialog.Accepted: # type: ignore
            # Create widgets for new move
            moveEdit = QLineEdit(nameEdit.text())
            damageSpin = QSpinBox()
            damageSpin.setRange(0, 100)

            # Add new move widgets to layout and lists
            self.moveEdits.append(moveEdit)
            self.damageSpins.append(damageSpin)
            movesLayout = QVBoxLayout()
            for move, damage in zip(self.moveEdits, self.damageSpins):
                movesLayout.addWidget(move)
                movesLayout.addWidget(damage)
            movesLayout.addWidget(self.addMoveButton)
            self.layout().itemAt(0).addItem(movesLayout)


    def getCardInfo(self):
        cardInfo: dict[str, typing.Any] = {
            "name": self.nameEdit.text(),
            "type": self.typeCombo.currentText(),
            "hp": self.hpSpin.value(),
            "moves": []
        }
        for moveEdit, damageSpin in zip(self.moveEdits, self.damageSpins):
            moveInfo: dict[str, typing.Any] = {}
            moveInfo["name"] = moveEdit.text()
            moveInfo["damage"] = damageSpin.value()
            cardInfo["moves"].append(moveInfo)
        return cardInfo

class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Pokemon Card Tracker")
        self.cards = []

        addButton = QPushButton("Add Card")

        # Create layout
        layout = QVBoxLayout()
        layout.addWidget(addButton)

        # Connect button signal to slot
        addButton.clicked.connect(self.showAddCardDialog)

        self.setLayout(layout)

    def showAddCardDialog(self):
        # Create dialog and get card info
        dialog = AddCardDialog()
        if dialog.exec_() == QDialog.Accepted:
            cardInfo = dialog.getCardInfo()
            self.cards.append(cardInfo)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())
