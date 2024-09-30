// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/commands/create_passenger_command.h"
#include "passenger/passenger_dto.h"
#include "repository/interface_passenger_repository.h"
#include "result.h"
#include "simple_example_application_passenger_export.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;

namespace Simple::Application::Features::Passenger::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_PASSENGER_EXPORT CreatePassengerCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreatePassengerCommandHandler(InterfacePassengerRepository *repository);

    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const CreatePassengerCommand &request);
    Result<PassengerDTO> restore();

  Q_SIGNALS:
    void passengerCreated(Simple::Contracts::DTO::Passenger::PassengerDTO passengerDto);
    void passengerRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfacePassengerRepository *m_repository;
    Result<PassengerDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreatePassengerCommand &request);
    Result<PassengerDTO> restoreImpl();
    Result<Simple::Entities::Passenger> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    QList<Simple::Entities::Passenger> m_oldOwnerPassengers;
    QList<Simple::Entities::Passenger> m_ownerPassengersNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Simple::Application::Features::Passenger::Commands