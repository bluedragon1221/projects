import sys
from PyQt5.QtCore import QFile, QRegExp, Qt
from PyQt5.QtGui import QColor, QTextCharFormat, QTextCursor, QTextDocument, QSyntaxHighlighter, QFont
from PyQt5.QtWidgets import QApplication, QMainWindow, QTextEdit, QAction, QFileDialog


class SyntaxHighlighter(QSyntaxHighlighter):
    def __init__(self, document):
        super().__init__(document)

        self.highlightingRules = []

        keywordFormat = QTextCharFormat()
        keywordFormat.setForeground(QColor(129, 161, 193))
        keywordFormat.setFontWeight(QFont.Bold)
        keywords = ['and', 'as', 'assert', 'break', 'class', 'continue', 'def',
                    'del', 'elif', 'else', 'except', 'False', 'finally', 'for',
                    'from', 'global', 'if', 'import', 'in', 'is', 'lambda',
                    'None', 'nonlocal', 'not', 'or', 'pass', 'raise', 'return',
                    'True', 'try', 'while', 'with', 'yield']
        self.highlightingRules += [(QRegExp(r'\b' + keyword + r'\b'), keywordFormat)
                                   for keyword in keywords]

        classFormat = QTextCharFormat()
        classFormat.setForeground(QColor(143, 188, 187))
        classFormat.setFontWeight(QFont.Bold)
        self.highlightingRules.append((QRegExp(r'\b[A-Za-z0-9_]+(?=\s*:)\b'), classFormat))

        functionFormat = QTextCharFormat()
        functionFormat.setForeground(QColor(129, 161, 193))
        functionFormat.setFontItalic(True)
        self.highlightingRules.append((QRegExp(r'\b[A-Za-z0-9_]+(?=\()'), functionFormat))

        quotationFormat = QTextCharFormat()
        quotationFormat.setForeground(QColor(163, 190, 140))
        self.highlightingRules.append((QRegExp('\".*\"'), quotationFormat))
        self.highlightingRules.append((QRegExp('\'.*\''), quotationFormat))

        commentFormat = QTextCharFormat()
        commentFormat.setForeground(QColor(76, 86, 106))
        self.highlightingRules.append((QRegExp(r'\#.*'), commentFormat))

    def highlightBlock(self, text):
        for pattern, format in self.highlightingRules:
            expression = QRegExp(pattern)
            index = expression.indexIn(text)
            while index >= 0:
                length = expression.matchedLength()
                self.setFormat(index, length, format)
                index = expression.indexIn(text, index + length)

        self.setCurrentBlockState(0)

class TextEditor(QMainWindow):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        self.text = QTextEdit()
        self.text.setFontFamily("Monospace")
        self.highlighter = SyntaxHighlighter(self.text.document())
        self.setCentralWidget(self.text)
        self.statusBar()

        openFile = QAction('Open', self)
        openFile.setShortcut('Ctrl+O')
        openFile.setStatusTip('Open file')
        openFile.triggered.connect(self.showDialog)

        saveFile = QAction('Save', self)
        saveFile.setShortcut('Ctrl+S')
        saveFile.setStatusTip('Save file')
        saveFile.triggered.connect(self.saveFileDialog)

        menubar = self.menuBar()
        fileMenu = menubar.addMenu('File')
        fileMenu.addAction(openFile)
        fileMenu.addAction(saveFile)

        self.setGeometry(300, 300, 350, 300)
        self.setWindowTitle('Text Editor')

        # Load the QSS from file
        style_file = QFile("style.qss")
        style_file.open(QFile.ReadOnly | QFile.Text)
        stream = style_file.readAll()
        style_file.close()
        qss = bytes(stream).decode("utf-8")

        # Set the stylesheet for the application
        self.setStyleSheet(qss)

    def showDialog(self):
        fname = QFileDialog.getOpenFileName(self, 'Open file', '/home')[0]
        if fname:
            with open(fname, 'r') as f:
                file_text = f.read()
                self.text.setText(file_text)

    def saveFileDialog(self):
        fname = QFileDialog.getSaveFileName(self, 'Save file', '/home')[0]
        if fname:
            with open(fname, 'w') as f:
                file_text = self.text.toPlainText()
                f.write(file_text)

if __name__ == '__main__':
    app = QApplication(sys.argv)
    editor = TextEditor()
    editor.show()
    sys.exit(app.exec_())