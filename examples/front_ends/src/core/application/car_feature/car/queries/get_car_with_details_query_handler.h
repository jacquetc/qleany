// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_with_details_dto.h"
#include "car/queries/get_car_query.h"
#include "front_ends_example_application_car_export.h"

#include "repository/interface_car_repository.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Car::Queries;

namespace FrontEnds::Application::Features::Car::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CAR_EXPORT GetCarWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
public:
    GetCarWithDetailsQueryHandler(InterfaceCarRepository *repository);
    Result<CarWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetCarQuery &query);

private:
    InterfaceCarRepository *m_repository;
    Result<CarWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetCarQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Car::Queries