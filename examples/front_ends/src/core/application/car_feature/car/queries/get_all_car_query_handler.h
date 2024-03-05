// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "front_ends_example_application_car_export.h"

#include "repository/interface_car_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Application::Features::Car::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CAR_EXPORT GetAllCarQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllCarQueryHandler(InterfaceCarRepository *repository);
    Result<QList<CarDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceCarRepository *m_repository;
    Result<QList<CarDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Car::Queries