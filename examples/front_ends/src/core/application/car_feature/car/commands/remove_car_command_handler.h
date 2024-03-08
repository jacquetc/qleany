// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/commands/remove_car_command.h"
#include "front_ends_example_application_car_export.h"

#include "repository/interface_car_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Car::Commands;

namespace FrontEnds::Application::Features::Car::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CAR_EXPORT RemoveCarCommandHandler : public QObject
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
    FrontEnds::Entities::Car m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Car::Commands