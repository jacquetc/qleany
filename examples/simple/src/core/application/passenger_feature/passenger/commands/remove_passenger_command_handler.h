// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_passenger_export.h"
#include "passenger/commands/remove_passenger_command.h"
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
class SIMPLEEXAMPLE_APPLICATION_PASSENGER_EXPORT RemovePassengerCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemovePassengerCommandHandler(InterfacePassengerRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemovePassengerCommand &request);
    Result<int> restore();

  signals:
    // repositories handle remove signals
    // void passengerRemoved(int id);

  private:
    InterfacePassengerRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemovePassengerCommand &request);
    Result<int> restoreImpl();
    Simple::Domain::Passenger m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Passenger::Commands