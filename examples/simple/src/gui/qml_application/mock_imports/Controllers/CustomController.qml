// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {


    function writeRandomThings(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.custom().writeRandomThingsChanged(reply_dto);
            })
        }

        return task
    }


    function runLongOperation(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.custom().runLongOperationChanged(reply_dto);
            })
        }

        return task
    }


    function closeSystem(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.custom().closeSystemChanged(reply_dto);
            })
        }

        return task
    }


    function getCurrentTime(dto) {
        // change this function to return the correct signal name, dto in and dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(controller);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
                EventDispatcher.custom().getCurrentTimeReplied(reply_dto);
            })
        }

        return task
    }


}