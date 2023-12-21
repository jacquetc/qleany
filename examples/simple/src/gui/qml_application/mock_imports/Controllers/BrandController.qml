// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {


    function getCreateDTO() {
        return {
            "content": "Brand 1"
        }
    }

    function create(dto) {
        // create random id
        var newId = Math.floor(Math.random() * 1000000);
        dto["id"] = newId;

        // mocking QCoro::Task
        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 50; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const result = dto;
                                                           if (result) {
                                                               EventDispatcher.brand().created(dto);
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${dto}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });
    }

    function get(id) {
        // mocking QCoro::Task
        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 50; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const result = dto;
                                                           if (result) {
                                                               EventDispatcher.brand().getReplied(id);
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${id}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });
    }

    function getAll() {

        // fill it with whatever you want to return
        var dtos = []

        // mocking QCoro::Task
        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 50; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const result = dtos;
                                                           if (result) {
                                                               EventDispatcher.brand().getAllReplied(dtos);
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${dtos}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });

    }

    function getUpdateDTO() {
        return {
            "id": 0,
            "content": ""
        }
    }

    function update(dto) {


        // mocking QCoro::Task
        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 50; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const result = dto;
                                                           if (result) {
                                                               EventDispatcher.brand().updated(dto);
                                                               EventDispatcher.brand().allRelationsInvalidated(dto.id);
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${dto}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });
    }

    signal brandRemoved(int id)
    function remove(id) {
        
        // mocking QCoro::Task
        return new Promise((resolve, reject) => {
                               var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                               timer.interval = 50; // delay
                               timer.repeat = false;
                               timer.triggered.connect(() => {
                                                           const result = true;
                                                           if (result) {
                                                               EventDispatcher.brand().removed(id);
                                                               resolve(result);
                                                           } else {
                                                               reject(new Error(`No value found for ${id}`));
                                                           }
                                                           timer.destroy(); // Clean up the timer
                                                       });
                               timer.start();
                           });
    }


}