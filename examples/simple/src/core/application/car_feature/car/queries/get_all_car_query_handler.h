// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_car_export.h"
#include "car/car_dto.h"

#include "repository/interface_car_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;

namespace Simple::Application::Features::Car::Queries
{
class SIMPLEEXAMPLE_APPLICATION_CAR_EXPORT GetAllCarQueryHandler : public QObject
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

} // namespace Simple::Application::Features::Car::Queries