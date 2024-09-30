// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "car.h"
#include "client.h"
#include "entity.h"
#include "passenger.h"

#include <QObject>

namespace FrontEnds::Entities
{

class EntitiesRegistration : public QObject
{
    Q_OBJECT
public:
    explicit EntitiesRegistration(QObject *parent)
        : QObject(parent)
    {
        qRegisterMetaType<FrontEnds::Entities::Entity>("FrontEnds::Entities::Entity");
        qRegisterMetaType<FrontEnds::Entities::Car>("FrontEnds::Entities::Car");
        qRegisterMetaType<FrontEnds::Entities::Brand>("FrontEnds::Entities::Brand");
        qRegisterMetaType<FrontEnds::Entities::Passenger>("FrontEnds::Entities::Passenger");
        qRegisterMetaType<FrontEnds::Entities::Client>("FrontEnds::Entities::Client");
    }
};

} // namespace FrontEnds::Entities