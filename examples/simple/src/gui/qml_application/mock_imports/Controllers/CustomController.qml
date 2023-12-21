// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {


    function writeRandomThings(dto) {
    // change this function to return the correct signal name, dto in and dto out

    reply_dto = {
        "id": 0,
        "content": ""
    }

    // mocking QCoro::Task
    return new Promise((resolve, reject) => {
                            var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                            timer.interval = 50; // delay
                            timer.repeat = false;
                            timer.triggered.connect(() => {
                                                        const result = reply_dto;
                                                        if (result) {
                                                            EventDispatcher.custom().writeRandomThingsChanged(reply_dto);
                                                            resolve(result);
                                                        } else {
                                                            reject(new Error(`No value found for ${reply_dto}`));
                                                        }
                                                        timer.destroy(); // Clean up the timer
                                                    });
                            timer.start();
                        });
    }


    function runLongOperation(dto) {
    // change this function to return the correct signal name, dto in and dto out

    reply_dto = {
        "id": 0,
        "content": ""
    }

    // mocking QCoro::Task
    return new Promise((resolve, reject) => {
                            var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                            timer.interval = 50; // delay
                            timer.repeat = false;
                            timer.triggered.connect(() => {
                                                        const result = reply_dto;
                                                        if (result) {
                                                            EventDispatcher.custom().runLongOperationChanged(reply_dto);
                                                            resolve(result);
                                                        } else {
                                                            reject(new Error(`No value found for ${reply_dto}`));
                                                        }
                                                        timer.destroy(); // Clean up the timer
                                                    });
                            timer.start();
                        });
    }


    function closeSystem(dto) {
    // change this function to return the correct signal name, dto in and dto out

    reply_dto = {
        "id": 0,
        "content": ""
    }

    // mocking QCoro::Task
    return new Promise((resolve, reject) => {
                            var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                            timer.interval = 50; // delay
                            timer.repeat = false;
                            timer.triggered.connect(() => {
                                                        const result = reply_dto;
                                                        if (result) {
                                                            EventDispatcher.custom().closeSystemChanged(reply_dto);
                                                            resolve(result);
                                                        } else {
                                                            reject(new Error(`No value found for ${reply_dto}`));
                                                        }
                                                        timer.destroy(); // Clean up the timer
                                                    });
                            timer.start();
                        });
    }


    function getCurrentTime(dto) {
    // change this function to return the correct signal name, dto in and dto out

    reply_dto = {
        "id": 0,
        "content": ""
    }

    // mocking QCoro::Task
    return new Promise((resolve, reject) => {
                            var timer = Qt.createQmlObject('import QtQuick 2.0; Timer {}', Qt.application);
                            timer.interval = 50; // delay
                            timer.repeat = false;
                            timer.triggered.connect(() => {
                                                        const result = reply_dto;
                                                        if (result) {
                                                            EventDispatcher.custom().getCurrentTimeReplied(reply_dto);
                                                            resolve(result);
                                                        } else {
                                                            reject(new Error(`No value found for ${reply_dto}`));
                                                        }
                                                        timer.destroy(); // Clean up the timer
                                                    });
                            timer.start();
                        });
    }


}