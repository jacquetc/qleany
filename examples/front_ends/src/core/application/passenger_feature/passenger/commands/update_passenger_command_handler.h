// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_application_passenger_export.h"
#include "passenger/commands/update_passenger_command.h"
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
class FRONT_ENDS_EXAMPLE_APPLICATION_PASSENGER_EXPORT UpdatePassengerCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdatePassengerCommandHandler(InterfacePassengerRepository *repository);
    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const UpdatePassengerCommand &request);
    Result<PassengerDTO> restore();

  Q_SIGNALS:
    void passengerUpdated(FrontEnds::Contracts::DTO::Passenger::PassengerDTO passengerDto);
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

} // namespace FrontEnds::Application::Features::Passenger::Commands