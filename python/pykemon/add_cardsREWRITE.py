import sys
import json
from PyQt5.QtWidgets import (
    QApplication, QWidget, QLabel, QLineEdit, QComboBox, QSpinBox,
    QPushButton, QVBoxLayout, QHBoxLayout, QDialog, QFormLayout
)

# Main Window
class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        self.cards = []

        ## Basic App Layout
        layout = QVBoxLayout()
        self.setWindowTitle("Pykemon")

        ## "Add Card" Button
        addCardButton = QPushButton("Add Card")
        addCardButton.clicked.connect(self.showAddCardDialog)
        layout.addWidget(addCardButton)
    
        self.setLayout(layout)
 
    def showAddCardDialog(self):
        """
        Function to open the "Add Card" window
        """
        dialog = AddCardDialog()
        if dialog.exec_() == QDialog.Accepted:
            cardInfo = dialog.getCardInfo()
            self.cards.append(cardInfo)

# Add Card Dialog
class AddCardDialog(QDialog):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Add Card")
        
        ## Basic Form Layout
        formLayout = QVBoxLayout()

        ### Input Name
        nameLabel = QLabel("Name:")
        formLayout.addWidget(nameLabel)
        nameInput = QLineEdit()
        formLayout.addWidget(nameInput)

        ### Input Type
        typeLabel = QLabel("Type:")
        formLayout.addWidget(typeLabel)
        typeSelect = QComboBox()
        typeSelect.addItems(["Normal", "Fire", "Water", "Grass", "Electric", "Dark"])
        formLayout.addWidget(typeSelect)

        ### Input HP
        hpLabel = QLabel("HP:")
        formLayout.addWidget(hpLabel)
        hpNum = QSpinBox()
        hpNum.setRange(0, 500)
        formLayout.addWidget(hpNum)

        ### Add Moves
        movesLayout = QVBoxLayout()

        #### Moves Label
        movesLabel = QLabel("Moves")
        movesLayout.addWidget(movesLabel)

        addMoveButton = QPushButton("Add Move")
        addMoveButton.clicked.connect(self.showAddMoveDialog)

        addMoveButton = QPushButton("Add Move")

        addCardButton = QPushButton("Done")
        
    def showAddMoveDialog(self):
        """
        Function to open the "Add Card" window
        """
        dialog = AddMoveDialog()
        if dialog.exec_() == QDialog.Accepted:
            cardInfo = dialog.getCardInfo()
            self.cards.append(cardInfo)

class AddMoveDialog(QDialog):
    pass