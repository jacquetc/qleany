// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_application_passenger_export.h"
#include "passenger/passenger_dto.h"
#include "passenger/queries/get_passenger_query.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Passenger;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Passenger::Queries;

namespace FrontEnds::Application::Features::Passenger::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_PASSENGER_EXPORT GetPassengerQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetPassengerQueryHandler(InterfacePassengerRepository *repository);
    Result<PassengerDTO> handle(QPromise<Result<void>> &progressPromise, const GetPassengerQuery &query);

  private:
    InterfacePassengerRepository *m_repository;
    Result<PassengerDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetPassengerQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Passenger::Queries