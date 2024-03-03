#pragma once
#include "single_brand.h"
#include <QQmlEngine>

struct ForeignSingleBrand
{
    Q_GADGET
    QML_FOREIGN(Simple::Presenter::SingleBrand)
    QML_NAMED_ELEMENT(SingleBrand)
};