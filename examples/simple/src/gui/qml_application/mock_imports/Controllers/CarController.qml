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
