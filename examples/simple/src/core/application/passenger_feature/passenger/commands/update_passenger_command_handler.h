// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/commands/update_passenger_command.h"
#include "passenger/passenger_dto.h"
#include "simple_example_application_passenger_export.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;

namespace Simple::Application::Features::Passenger::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_PASSENGER_EXPORT UpdatePassengerCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdatePassengerCommandHandler(InterfacePassengerRepository *repository);
    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const UpdatePassengerCommand &request);
    Result<PassengerDTO> restore();

  Q_SIGNALS:
    void passengerUpdated(Simple::Contracts::DTO::Passenger::PassengerDTO passengerDto);
    void passengerDetailsUpdated(int id);

  private:
    InterfacePassengerRepository *m_repository;
    Result<PassengerDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdatePassengerCommand &request);
    Result<PassengerDTO> restoreImpl();
    Result<PassengerDTO> saveOldState();
    Result<PassengerDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Passenger::Commands