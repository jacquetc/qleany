// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include "simple_example_application_passenger_export.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;

namespace Simple::Application::Features::Passenger::Queries
{
class SIMPLE_EXAMPLE_APPLICATION_PASSENGER_EXPORT GetAllPassengerQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllPassengerQueryHandler(InterfacePassengerRepository *repository);
    Result<QList<PassengerDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfacePassengerRepository *m_repository;
    Result<QList<PassengerDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Passenger::Queries