// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "domain_export.h"
#include "entity.h"
#include "car.h"
#include "brand.h"
#include "passenger.h"
#include "client.h"

#include <QObject>

namespace Simple::Domain {

class SIMPLEEXAMPLE_DOMAIN_EXPORT DomainRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit DomainRegistration(QObject *parent)
    {
        
        qRegisterMetaType<Simple::Domain::Entity>("Simple::Domain::Entity");
        qRegisterMetaType<Simple::Domain::Car>("Simple::Domain::Car");
        qRegisterMetaType<Simple::Domain::Brand>("Simple::Domain::Brand");
        qRegisterMetaType<Simple::Domain::Passenger>("Simple::Domain::Passenger");
        qRegisterMetaType<Simple::Domain::Client>("Simple::Domain::Client");

    }
};

} // namespace Simple::Domain