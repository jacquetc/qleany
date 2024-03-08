// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/commands/create_car_command.h"
#include "front_ends_example_application_car_export.h"
#include "repository/interface_car_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Entities;
using namespace FrontEnds::Contracts::DTO::Car;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Car::Commands;

namespace FrontEnds::Application::Features::Car::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CAR_EXPORT CreateCarCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateCarCommandHandler(InterfaceCarRepository *repository);

    Result<CarDTO> handle(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restore();

  Q_SIGNALS:
    void carCreated(FrontEnds::Contracts::DTO::Car::CarDTO carDto);
    void carRemoved(int id);

  private:
    InterfaceCarRepository *m_repository;
    Result<CarDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request);
    Result<CarDTO> restoreImpl();
    Result<FrontEnds::Entities::Car> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace FrontEnds::Application::Features::Car::Commands