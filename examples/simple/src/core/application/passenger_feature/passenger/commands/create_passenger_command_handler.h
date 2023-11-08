// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_passenger_export.h"
#include "passenger/commands/create_passenger_command.h"
#include "passenger/passenger_dto.h"
#include "repository/interface_passenger_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;

namespace Simple::Application::Features::Passenger::Commands
{
class SIMPLEEXAMPLE_APPLICATION_PASSENGER_EXPORT CreatePassengerCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreatePassengerCommandHandler(InterfacePassengerRepository *repository);

    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const CreatePassengerCommand &request);
    Result<PassengerDTO> restore();

  signals:
    void passengerCreated(Simple::Contracts::DTO::Passenger::PassengerDTO passengerDto);
    void passengerRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfacePassengerRepository *m_repository;
    Result<PassengerDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreatePassengerCommand &request);
    Result<PassengerDTO> restoreImpl();
    Result<Simple::Domain::Passenger> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    QList<Simple::Domain::Passenger> m_oldOwnerPassengers;
    QList<Simple::Domain::Passenger> m_ownerPassengersNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Simple::Application::Features::Passenger::Commands