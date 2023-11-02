// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_passenger_export.h"
#include "passenger/commands/update_passenger_command.h"
#include "passenger/passenger_dto.h"

#include "qleany/common/result.h"
#include "repository/interface_passenger_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;

namespace Simple::Application::Features::Passenger::Commands
{
class SIMPLEEXAMPLE_APPLICATION_PASSENGER_EXPORT UpdatePassengerCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdatePassengerCommandHandler(InterfacePassengerRepository *repository);
    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const UpdatePassengerCommand &request);
    Result<PassengerDTO> restore();

  signals:
    void passengerUpdated(Simple::Contracts::DTO::Passenger::PassengerDTO passengerDto);
    void passengerDetailsUpdated(int id);

  private:
    InterfacePassengerRepository *m_repository;
    Result<PassengerDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdatePassengerCommand &request);
    Result<PassengerDTO> restoreImpl();
    Result<PassengerDTO> saveOldState();
    Result<PassengerDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Passenger::Commands