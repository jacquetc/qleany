// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "car/car_with_details_dto.h"
#include "car/create_car_dto.h"
#include "car/update_car_dto.h"
#include "event_dispatcher.h"
#include "repository/interface_repository_provider.h"
#include "simple_example_controller_export.h"

#include "undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>

using namespace Simple::Contracts::Repository;
using namespace Simple::Controller::UndoRedo;
using namespace Simple::Contracts::DTO::Car;

namespace Simple::Controller::Car
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT CarController : public QObject
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

  public Q_SLOTS:

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