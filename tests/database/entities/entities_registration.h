// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "dummy_entity.h"
#include "dummy_basic_entity.h"
#include "dummy_other_entity.h"
#include "dummy_entity_with_foreign.h"

#include <QObject>

namespace DatabaseTest::Entities {

class EntitiesRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit EntitiesRegistration(QObject *parent) : QObject(parent)
    {
        
        qRegisterMetaType<DatabaseTest::Entities::DummyEntity>("DatabaseTest::Entities::DummyEntity");
        qRegisterMetaType<DatabaseTest::Entities::DummyBasicEntity>("DatabaseTest::Entities::DummyBasicEntity");
        qRegisterMetaType<DatabaseTest::Entities::DummyOtherEntity>("DatabaseTest::Entities::DummyOtherEntity");
        qRegisterMetaType<DatabaseTest::Entities::DummyEntityWithForeign>("DatabaseTest::Entities::DummyEntityWithForeign");

    }
};

} // namespace DatabaseTest::Entities