#pragma once
#include "single_client.h"
#include <QQmlEngine>

struct ForeignSingleClient
{
    Q_GADGET
    QML_FOREIGN(Simple::Presenter::SingleClient)
    QML_NAMED_ELEMENT(SingleClient)
};