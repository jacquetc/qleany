// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    id: interactor


    function get(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.car().getReplied(id)})
        }

        return task
    }

    function getWithDetails(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.car().getWithDetailsReplied(id)})
        }

        return task
    }

    function getAll() {
        // fill it with whatever you want to return
        var dtos = []

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dtos);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.car().getAllReplied(dtos)})
        }

        return task
    }

    function getCreateDTO() {
        return {
            "content": "Car 1"
        }
    }

    function create(dto) {
        // create random id
        var newId = Math.floor(Math.random() * 1000000);
        dto["id"] = newId;

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){EventDispatcher.car().created(dto)})
        }

        return task
    }

    function getUpdateDTO() {
        return {
            "id": 0,
            "content": ""
        }
    }

    function update(dto) {

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.car().updated(dto);
                EventDispatcher.car().allRelationsInvalidated(dto.id);
            })
        }

        return task
    }

    signal carRemoved(int id)
    function remove(id) {
        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.car().removed(id);
            })
        }

        return task
    }


}