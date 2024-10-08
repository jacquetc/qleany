// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/commands/update_car_command.h"
#include "simple_example_application_car_export.h"

#include "repository/interface_car_repository.h"
#include "result.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Commands;

namespace Simple::Application::Features::Car::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_CAR_EXPORT UpdateCarCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateCarCommandHandler(InterfaceCarRepository *repository);
    Result<CarDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateCarCommand &request);
    Result<CarDTO> restore();

  Q_SIGNALS:
    void carUpdated(Simple::Contracts::DTO::Car::CarDTO carDto);
    void carDetailsUpdated(int id);

  private:
    InterfaceCarRepository *m_repository;
    Result<CarDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateCarCommand &request);
    Result<CarDTO> restoreImpl();
    Result<CarDTO> saveOldState();
    Result<CarDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Car::Commands