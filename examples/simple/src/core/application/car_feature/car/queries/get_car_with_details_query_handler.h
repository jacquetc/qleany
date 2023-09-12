// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_car_export.h"
#include "car/car_with_details_dto.h"
#include "car/queries/get_car_query.h"

#include "repository/interface_car_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Queries;

namespace Simple::Application::Features::Car::Queries
{
class SIMPLEEXAMPLE_APPLICATION_CAR_EXPORT GetCarWithDetailsQueryHandler : public QObject
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

} // namespace Simple::Application::Features::Car::Queries