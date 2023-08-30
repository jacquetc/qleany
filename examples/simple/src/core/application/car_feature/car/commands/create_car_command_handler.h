// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_car_export.h"
#include "car/car_dto.h"
#include "car/commands/create_car_command.h"
#include "qleany/common/result.h"
#include "repository/interface_car_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;

namespace Simple::Application::Features::Car::Commands
{
class SIMPLEEXAMPLE_APPLICATION_CAR_EXPORT CreateCarCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateCarCommandHandler(InterfaceCarRepository *repository);

    Result<CarDTO> handle(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restore();

  signals:
    void carCreated(Simple::Contracts::DTO::Car::CarDTO carDto);
    void carRemoved(int id);

  private:
    InterfaceCarRepository *m_repository;
    Result<CarDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restoreImpl();
    Result<Simple::Domain::Car> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Car::Commands