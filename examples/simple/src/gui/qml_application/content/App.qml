// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR GPL-3.0-only
import QtQuick 6.5
import SimpleExampleQML
import Controllers
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls 6.5

Window {

    visible: true
    title: "SimpleExampleQML"

    // message box component
    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        Screen01 {
            id: mainScreen
            Layout.fillWidth: true
            Layout.fillHeight: true
        }

        Rectangle {
            id: progressBarRectangle
            Layout.fillWidth: true
            Layout.minimumHeight: 40
            visible: false

            ColumnLayout {
                id: columnLayout2
                anchors.fill: parent

                Label {
                    id: progressBarText
                    Layout.fillWidth: true
                    Layout.minimumHeight: 20
                }
                ProgressBar {
                    id: progressBar

                    Layout.fillWidth: true
                    Layout.minimumHeight: 20
                }
            }
        }
    }
    Connections {
        target: EventDispatcher.progress()
        function onProgressTextChanged(text) {
            progressBarText.text = "Progress text: " + text
            console.log("Progress text: " + text)
        }
    }

    Connections {
        target: EventDispatcher.progress()
        function onProgressValueChanged(value) {
            progressBar.value = value
            console.log("Progress value: " + value)
        }
    }

    Connections {
        target: EventDispatcher.progress()
        function onProgressFinished() {
            progressBarRectangle.visible = false
        }
    }

    Connections {
        target: EventDispatcher.progress()
        function onProgressRangeChanged(minimum, maximum) {
            progressBar.minimumValue = minimum
            progressBar.maximumValue = maximum
            console.log("Progress range: " + minimum + " - " + maximum)
        }
    }

    Connections {
        target: EventDispatcher.progress()
        function onProgressStarted() {
            progressBarRectangle.visible = true
        }
    }

    Component {
        id: messageBoxComponent
        MessageDialog {

            onAccepted: {
                console.log("Message accepted")
            }
        }
    }

    // connnect to error signals and display a message box
    // warningSent
    Connections {
        target: EventDispatcher.error()
        function onWarningSent(error) {
            // using messageBoxComponent
            var messageBox = messageBoxComponent.createObject(mainScreen)
            messageBox.title = "Warning"
            messageBox.text = "Warning received: " + error.code()
            messageBox.detailedText = error.message()
            messageBox.informativeText = error.data()
            messageBox.open()

            console.log("Warning received: " + error.code(
                            ) + " : " + error.message())
        }
    }

    // errorSent
    Connections {
        target: EventDispatcher.error()
        function onErrorSent(error) {
            // using messageBoxComponent
            var messageBox = messageBoxComponent.createObject(mainScreen)
            messageBox.title = "Error"
            messageBox.text = "Error received: " + error.code()
            messageBox.detailedText = error.message()
            messageBox.informativeText = error.data()
            messageBox.open()

            console.log("Error received: " + error.code(
                            ) + " : " + error.message())
        }
    }


}
