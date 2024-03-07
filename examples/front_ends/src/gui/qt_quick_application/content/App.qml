// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

// Components
import Interactors
import Models
import Singles

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: FrontEndsExample


    // Main layout
    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        // Header
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            // Title
            Text {
                text: "Hello World"
                font.pixelSize: 20
            }

            // Spacer
            Item {
                Layout.fillWidth: true
            }

            // Button
            Button {
                text: "Click me"
                onClicked: {
                    console.log("Button clicked")
                }
            }
        }

        // Content
        Text {
            text: "Hello World"
        }
    }
}