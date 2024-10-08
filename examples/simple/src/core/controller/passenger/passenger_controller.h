// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "event_dispatcher.h"
#include "passenger/create_passenger_dto.h"
#include "passenger/passenger_dto.h"
#include "passenger/update_passenger_dto.h"
#include "repository/interface_repository_provider.h"
#include "simple_example_controller_export.h"

#include "undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>

using namespace Simple::Contracts::Repository;
using namespace Simple::Controller::UndoRedo;
using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Controller::Passenger
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT PassengerController : public QObject
{
    Q_OBJECT
  public:
    explicit PassengerController(InterfaceRepositoryProvider *repositoryProvider,
                                 ThreadedUndoRedoSystem *undo_redo_system,
                                 QSharedPointer<EventDispatcher> eventDispatcher);

    static PassengerController *instance();

    Q_INVOKABLE QCoro::Task<PassengerDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<QList<PassengerDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Passenger::CreatePassengerDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Passenger::UpdatePassengerDTO getUpdateDTO();

  public Q_SLOTS:

    QCoro::Task<PassengerDTO> create(const CreatePassengerDTO &dto);

    QCoro::Task<PassengerDTO> update(const UpdatePassengerDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<PassengerController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    PassengerController() = delete;
    PassengerController(const PassengerController &) = delete;
    PassengerController &operator=(const PassengerController &) = delete;
};

} // namespace Simple::Controller::Passenger