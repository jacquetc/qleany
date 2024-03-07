// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
pragma Singleton

import QtQuick

QtObject {
    function progress() {
        return ProgressSignals;
    }
    function error() {
        return ErrorSignals;
    }

    function undoRedo() {
        return UndoRedoSignals;
    }



    function car() {
        return CarSignals;
    }

    function brand() {
        return BrandSignals;
    }

    function passenger() {
        return PassengerSignals;
    }

    function client() {
        return ClientSignals;
    }

    function custom() {
        return CustomSignals;
    }

}