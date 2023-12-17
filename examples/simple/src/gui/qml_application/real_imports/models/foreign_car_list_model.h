#pragma once
#include "car_list_model.h"
#include <QQmlEngine>

struct ForeignCarListModel
{
    Q_GADGET
    QML_FOREIGN(Simple::Presenter::CarListModel)
    QML_NAMED_ELEMENT(CarListModel)
};
