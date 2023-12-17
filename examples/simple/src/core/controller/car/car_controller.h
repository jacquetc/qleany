// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/car_with_details_dto.h"
#include "car/create_car_dto.h"
#include "car/update_car_dto.h"
#include "controller_export.h"
#include "event_dispatcher.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Car;

namespace Simple::Controller::Car
{

class SIMPLEEXAMPLE_CONTROLLER_EXPORT CarController : public QObject
{
    Q_OBJECT
  public:
    explicit CarController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                           QSharedPointer<EventDispatcher> eventDispatcher);

    static CarController *instance();

    Q_INVOKABLE QCoro::Task<CarDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<CarWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE QCoro::Task<QList<CarDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Car::CreateCarDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Car::UpdateCarDTO getUpdateDTO();

  public slots:

    QCoro::Task<CarDTO> create(const CreateCarDTO &dto);

    QCoro::Task<CarDTO> update(const UpdateCarDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<CarController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    CarController() = delete;
    CarController(const CarController &) = delete;
    CarController &operator=(const CarController &) = delete;
};

} // namespace Simple::Controller::Car