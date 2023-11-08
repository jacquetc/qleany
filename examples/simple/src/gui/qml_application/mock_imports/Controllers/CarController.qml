pragma Singleton
import QtQuick

QtObject {

    function getCreateDTO() {
        return {
            "content": ""
        };
    }
    function getUpdateDTO() {
        return {
            "id": 0,
            "content": ""
        };
    }

    function get(id) {
        EventDispatcher.car().getReplied(id);
    }

    function getWithDetails(id) {
        EventDispatcher.car().getWithDetailsReplied(id);
    }

    function create(dto) {
        dto["id"] = 1;

        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 500; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const mockDatabaseValues = {
                                                               'key': 'mockedValue', // Replace with your mock data
                                                               // ... other keys
                                                           };

                                                           const result = dto;
                                                           if (result) {
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${dto}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });

    }

    function update(dto) {
        EventDispatcher.car().updated(dto);
        EventDispatcher.car().detailsUpdated(dto);
    }

    function remove(id) {
        EventDispatcher.car().removed(id);
    }
}
