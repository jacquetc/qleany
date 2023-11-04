

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick 6.5
import QtQuick.Controls 6.5
import SimpleExampleQML
import QtQuick.Layouts
import Models

Rectangle {
    id: rectangle

    color: Constants.backgroundColor
    property alias carPassengersListView: carPassengersListView
    property alias carListView: carListView
    property alias addCarButton: addCarButton
    property alias addPassengerInCarButton: addPassengerInCarButton
    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        RowLayout {
            id: rowLayout1
            ListView {
                id: carListView
                Layout.fillWidth: true
                model: CarListModel {}
                delegate: ItemDelegate {
                    property int itemId: model.itemId
                    text: model.content
                    highlighted: ListView.isCurrentItem
                    onClicked: ListView.view.currentIndex = index
                }
                Layout.fillHeight: true
            }

            Button {
                id: addCarButton
                text: qsTr("Add car")
            }
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        RowLayout {
            id: rowLayout
            Layout.fillHeight: true
            Layout.fillWidth: true

            ListView {
                id: carPassengersListView
                Layout.fillWidth: true
                Layout.fillHeight: true
                model: PassengerListModelFromCarPassengers {
                    carId: carListView.currentItem ? carListView.currentItem.itemId : 0
                }

                delegate: ItemDelegate {
                    property int itemId: model.itemId
                    text: model.name
                    highlighted: ListView.isCurrentItem
                    onClicked: ListView.view.currentIndex = index
                }
            }

            Button {
                id: addPassengerInCarButton
                text: qsTr("Add passenger in car")
            }
        }
    }
}
