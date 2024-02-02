import QtQuick 2.15
import QtQuick.Controls 2.15
import Interactors
import Singles

Screen01Form {

    property int currentCarId: carListView.currentItem ? carListView.currentItem.itemId : 0
    property int carNumber: 0
    property int passengerNumber: 0

    SingleCar {
        id: singleCar
        itemId: currentCarId
    }

    carLabel.text: singleCar.content

    Connections {
        target: runLongOperationButton
        function onClicked() {
            CustomInteractor.runLongOperation()
        }
    }

    Connections {
        target: addPassengerInCarButton
        function onClicked() {
            if (currentCarId === 0) {
                return
            }

            var dto = PassengerInteractor.getCreateDTO()
            dto.name = "p " + passengerNumber
            passengerNumber = passengerNumber + 1
            dto.carId = currentCarId
            dto.position = 0
            PassengerInteractor.create(dto)
        }
    }

    Connections {
        target: addCarButton
        function onClicked() {
            var dto = CarInteractor.getCreateDTO()
            dto.content = "c " + carNumber
            carNumber = carNumber + 1
            CarInteractor.create(dto).then(result => console.log(
                                               "Result", result.content))
        }
    }

}
