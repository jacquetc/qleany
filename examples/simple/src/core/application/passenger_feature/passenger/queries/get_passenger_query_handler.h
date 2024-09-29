// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include "passenger/queries/get_passenger_query.h"
#include "simple_example_application_passenger_export.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Queries;

namespace Simple::Application::Features::Passenger::Queries
{
class SIMPLE_EXAMPLE_APPLICATION_PASSENGER_EXPORT GetPassengerQueryHandler : public QObject
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

} // namespace Simple::Application::Features::Passenger::Queries