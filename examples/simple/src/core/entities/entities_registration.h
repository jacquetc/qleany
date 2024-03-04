// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "entity.h"
#include "car.h"
#include "brand.h"
#include "passenger.h"
#include "client.h"

#include <QObject>

namespace Simple::Entities {

class EntitiesRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit EntitiesRegistration(QObject *parent) : QObject(parent)
    {
        
        qRegisterMetaType<Simple::Entities::Entity>("Simple::Entities::Entity");
        qRegisterMetaType<Simple::Entities::Car>("Simple::Entities::Car");
        qRegisterMetaType<Simple::Entities::Brand>("Simple::Entities::Brand");
        qRegisterMetaType<Simple::Entities::Passenger>("Simple::Entities::Passenger");
        qRegisterMetaType<Simple::Entities::Client>("Simple::Entities::Client");

    }
};

} // namespace Simple::Entities
