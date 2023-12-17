#pragma once
#include "passenger_list_model_from_car_passengers.h"
#include <QQmlEngine>

struct ForeignPassengerListModelFromCarPassengers
{
    Q_GADGET
    QML_FOREIGN(Simple::Presenter::PassengerListModelFromCarPassengers)
    QML_NAMED_ELEMENT(PassengerListModelFromCarPassengers)
};
