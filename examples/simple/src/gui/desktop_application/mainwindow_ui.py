# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'mainwindow.ui'
##
## Created by: Qt User Interface Compiler version 6.5.2
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QBrush, QColor, QConicalGradient, QCursor,
    QFont, QFontDatabase, QGradient, QIcon,
    QImage, QKeySequence, QLinearGradient, QPainter,
    QPalette, QPixmap, QRadialGradient, QTransform)
from PySide6.QtWidgets import (QAbstractItemView, QApplication, QFormLayout, QGroupBox,
    QHBoxLayout, QLabel, QLineEdit, QListView,
    QMainWindow, QMenuBar, QPushButton, QSizePolicy,
    QSpinBox, QStatusBar, QVBoxLayout, QWidget)

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(736, 641)
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.horizontalLayout = QHBoxLayout(self.centralwidget)
        self.horizontalLayout.setObjectName(u"horizontalLayout")
        self.verticalLayout_3 = QVBoxLayout()
        self.verticalLayout_3.setObjectName(u"verticalLayout_3")
        self.groupBox_2 = QGroupBox(self.centralwidget)
        self.groupBox_2.setObjectName(u"groupBox_2")
        self.verticalLayout = QVBoxLayout(self.groupBox_2)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.carListView = QListView(self.groupBox_2)
        self.carListView.setObjectName(u"carListView")

        self.verticalLayout.addWidget(self.carListView)

        self.addCarPushButton = QPushButton(self.groupBox_2)
        self.addCarPushButton.setObjectName(u"addCarPushButton")

        self.verticalLayout.addWidget(self.addCarPushButton)

        self.removeCarPushButton = QPushButton(self.groupBox_2)
        self.removeCarPushButton.setObjectName(u"removeCarPushButton")

        self.verticalLayout.addWidget(self.removeCarPushButton)


        self.verticalLayout_3.addWidget(self.groupBox_2)

        self.groupBox = QGroupBox(self.centralwidget)
        self.groupBox.setObjectName(u"groupBox")
        self.verticalLayout_2 = QVBoxLayout(self.groupBox)
        self.verticalLayout_2.setObjectName(u"verticalLayout_2")
        self.passengerListView = QListView(self.groupBox)
        self.passengerListView.setObjectName(u"passengerListView")
        self.passengerListView.setEditTriggers(QAbstractItemView.EditKeyPressed)

        self.verticalLayout_2.addWidget(self.passengerListView)

        self.addPassengerPushButton = QPushButton(self.groupBox)
        self.addPassengerPushButton.setObjectName(u"addPassengerPushButton")

        self.verticalLayout_2.addWidget(self.addPassengerPushButton)


        self.verticalLayout_3.addWidget(self.groupBox)


        self.horizontalLayout.addLayout(self.verticalLayout_3)

        self.groupBox_3 = QGroupBox(self.centralwidget)
        self.groupBox_3.setObjectName(u"groupBox_3")
        self.verticalLayout_4 = QVBoxLayout(self.groupBox_3)
        self.verticalLayout_4.setObjectName(u"verticalLayout_4")
        self.formLayout = QFormLayout()
        self.formLayout.setObjectName(u"formLayout")
        self.idLabel = QLabel(self.groupBox_3)
        self.idLabel.setObjectName(u"idLabel")

        self.formLayout.setWidget(0, QFormLayout.LabelRole, self.idLabel)

        self.idSpinBox = QSpinBox(self.groupBox_3)
        self.idSpinBox.setObjectName(u"idSpinBox")

        self.formLayout.setWidget(0, QFormLayout.FieldRole, self.idSpinBox)

        self.nameLabel = QLabel(self.groupBox_3)
        self.nameLabel.setObjectName(u"nameLabel")

        self.formLayout.setWidget(1, QFormLayout.LabelRole, self.nameLabel)

        self.nameLineEdit = QLineEdit(self.groupBox_3)
        self.nameLineEdit.setObjectName(u"nameLineEdit")

        self.formLayout.setWidget(1, QFormLayout.FieldRole, self.nameLineEdit)


        self.verticalLayout_4.addLayout(self.formLayout)


        self.horizontalLayout.addWidget(self.groupBox_3)

        MainWindow.setCentralWidget(self.centralwidget)
        self.menubar = QMenuBar(MainWindow)
        self.menubar.setObjectName(u"menubar")
        self.menubar.setGeometry(QRect(0, 0, 736, 24))
        MainWindow.setMenuBar(self.menubar)
        self.statusbar = QStatusBar(MainWindow)
        self.statusbar.setObjectName(u"statusbar")
        MainWindow.setStatusBar(self.statusbar)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"MainWindow", None))
        self.groupBox_2.setTitle(QCoreApplication.translate("MainWindow", u"All cars", None))
        self.addCarPushButton.setText(QCoreApplication.translate("MainWindow", u"Add car", None))
        self.removeCarPushButton.setText(QCoreApplication.translate("MainWindow", u"Remove car", None))
        self.groupBox.setTitle(QCoreApplication.translate("MainWindow", u"Passengers of a car", None))
        self.addPassengerPushButton.setText(QCoreApplication.translate("MainWindow", u"Add passenger", None))
        self.groupBox_3.setTitle(QCoreApplication.translate("MainWindow", u"One passenger", None))
        self.idLabel.setText(QCoreApplication.translate("MainWindow", u"Id", None))
        self.nameLabel.setText(QCoreApplication.translate("MainWindow", u"Name: ", None))
    # retranslateUi

