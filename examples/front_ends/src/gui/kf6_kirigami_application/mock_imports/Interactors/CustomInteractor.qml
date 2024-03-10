// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    id: interactor


    function writeRandomThings(dto) {
        // TODO: change this dict below to conform to the command's dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
            // TODO: change this signal name to the correct one
                EventDispatcher.custom().writeRandomThingsChanged(reply_dto);
            })
        }

        return task
    }

    
    function getWriteRandomThingsDTO() {
    // TODO: change this dict below to conform to the command's dto in
        return {
            "id": 0,
            "content": ""
        }
    }
    



    function runLongOperation(dto) {
        // TODO: change this dict below to conform to the command's dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
            // TODO: change this signal name to the correct one
                EventDispatcher.custom().runLongOperationChanged(reply_dto);
            })
        }

        return task
    }

    



    function closeSystem(dto) {
        // TODO: change this dict below to conform to the command's dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
            // TODO: change this signal name to the correct one
                EventDispatcher.custom().closeSystemChanged(reply_dto);
            })
        }

        return task
    }

    



    function getCurrentTime(dto) {
        // TODO: change this dict below to conform to the query's dto out
        var reply_dto = {
            "id": 0,
            "content": ""
        }

        // mocking QCoro::Task
        var component = Qt.createComponent("QCoroQmlTask.qml");
        if (component.status === Component.Ready) {
            var task = component.createObject(interactor);
            task.setValue(reply_dto);
            task.setDelay(50);
            task.setSignalFn(function(){
            // TODO: change this signal name to the correct one
                EventDispatcher.custom().getCurrentTimeReplied(reply_dto);
            })
        }

        return task
    }

    



}