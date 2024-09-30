// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/commands/create_car_command.h"
#include "repository/interface_car_repository.h"
#include "result.h"
#include "simple_example_application_car_export.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;

namespace Simple::Application::Features::Car::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_CAR_EXPORT CreateCarCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateCarCommandHandler(InterfaceCarRepository *repository);

    Result<CarDTO> handle(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restore();

  Q_SIGNALS:
    void carCreated(Simple::Contracts::DTO::Car::CarDTO carDto);
    void carRemoved(int id);

  private:
    InterfaceCarRepository *m_repository;
    Result<CarDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restoreImpl();
    Result<Simple::Entities::Car> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Simple::Application::Features::Car::Commands