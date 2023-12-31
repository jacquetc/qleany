// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "passenger/passenger_dto.h"

#include <QObject>

namespace Simple::Controller
{

using namespace Simple::Contracts::DTO::Passenger;

class SIMPLEEXAMPLE_CONTROLLER_EXPORT PassengerSignals : public QObject
{
    Q_OBJECT
  public:
    explicit PassengerSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void getReplied(PassengerDTO dto);
    void getAllReplied(QList<PassengerDTO> dtoList);
    void created(PassengerDTO dto);
    void updated(Contracts::DTO::Passenger::PassengerDTO dto);
    void allRelationsInvalidated(int id);
};
} // namespace Simple::Controller