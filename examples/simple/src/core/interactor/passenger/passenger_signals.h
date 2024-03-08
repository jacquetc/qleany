// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_interactor_export.h"

#include "passenger/passenger_dto.h"

#include <QObject>

namespace Simple::Interactor
{

using namespace Simple::Contracts::DTO::Passenger;

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT PassengerSignals : public QObject
{
    Q_OBJECT
  public:
    explicit PassengerSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(PassengerDTO dto);
    void updated(PassengerDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(PassengerDTO dto);
    void getAllReplied(QList<PassengerDTO> dtoList);
};
} // namespace Simple::Interactor