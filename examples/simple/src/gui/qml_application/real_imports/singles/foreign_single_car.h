#pragma once
#include "single_car.h"
#include <QQmlEngine>

struct ForeignSingleCar
{
    Q_GADGET
    QML_FOREIGN(Simple::Presenter::SingleCar)
    QML_NAMED_ELEMENT(SingleCar)
};