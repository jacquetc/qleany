pragma Singleton
import QtQuick

QtObject {

    function getCreateDTO() {
        return {
            "name": "",
            carId: 0
        };
    }
    function getUpdateDTO() {
        return {
            "id": 0,
            "name": ""
        };
    }

    function get(id) {
        EventDispatcher.chapter().getReplied(id);
    }

    function getWithDetails(id) {
        EventDispatcher.chapter().getWithDetailsReplied(id);
    }

    function create(dto) {
        dto["id"] = 1;
        EventDispatcher.chapter().created(dto);
    }

    function update(dto) {
        EventDispatcher.chapter().updated(dto);
        EventDispatcher.chapter().detailsUpdated(dto);
    }

    function remove(id) {
        EventDispatcher.chapter().removed(id);
    }
}
