// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_application_passenger_export.h"
#include "passenger/commands/remove_passenger_command.h"
#include "passenger/passenger_dto.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Passenger;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Passenger::Commands;

namespace FrontEnds::Application::Features::Passenger::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_PASSENGER_EXPORT RemovePassengerCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemovePassengerCommandHandler(InterfacePassengerRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemovePassengerCommand &request);
    Result<int> restore();

  Q_SIGNALS:
    // repositories handle remove Q_SIGNALS
    // void passengerRemoved(int id);

  private:
    InterfacePassengerRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemovePassengerCommand &request);
    Result<int> restoreImpl();
    FrontEnds::Entities::Passenger m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Passenger::Commands