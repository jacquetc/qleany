// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/commands/remove_car_command.h"
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
class SIMPLE_EXAMPLE_APPLICATION_CAR_EXPORT RemoveCarCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveCarCommandHandler(InterfaceCarRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveCarCommand &request);
    Result<int> restore();

  Q_SIGNALS:
    // repositories handle remove Q_SIGNALS
    // void carRemoved(int id);

  private:
    InterfaceCarRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveCarCommand &request);
    Result<int> restoreImpl();
    Simple::Entities::Car m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Car::Commands