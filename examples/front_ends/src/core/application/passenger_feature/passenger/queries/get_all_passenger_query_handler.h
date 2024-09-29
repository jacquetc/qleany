// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_application_passenger_export.h"
#include "passenger/passenger_dto.h"

#include "repository/interface_passenger_repository.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Passenger;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Application::Features::Passenger::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_PASSENGER_EXPORT GetAllPassengerQueryHandler : public QObject
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

} // namespace FrontEnds::Application::Features::Passenger::Queries