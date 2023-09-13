// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "passenger/create_passenger_dto.h"
#include "passenger/passenger_dto.h"
#include "passenger/update_passenger_dto.h"
#include "qleany/contracts/repository/interface_repository_provider.h"

#include "qleany/tools/undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QSharedPointer>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Controller::Passenger
{

class SIMPLEEXAMPLE_CONTROLLER_EXPORT PassengerController : public QObject
{
    Q_OBJECT
  public:
    explicit PassengerController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                 ThreadedUndoRedoSystem *undo_redo_system,
                                 QSharedPointer<EventDispatcher> eventDispatcher);

    static PassengerController *instance();

  public slots:

    QCoro::Task<PassengerDTO> get(int id);

    QCoro::Task<QList<PassengerDTO>> getAll();

    QCoro::Task<PassengerDTO> create(const CreatePassengerDTO &dto);

    QCoro::Task<PassengerDTO> update(const UpdatePassengerDTO &dto);

    QCoro::Task<bool> remove(int id);

    static Contracts::DTO::Passenger::CreatePassengerDTO getCreateDTO();

    static Contracts::DTO::Passenger::UpdatePassengerDTO getUpdateDTO();

  private:
    static QScopedPointer<PassengerController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    PassengerController() = delete;
    PassengerController(const PassengerController &) = delete;
    PassengerController &operator=(const PassengerController &) = delete;
};

} // namespace Simple::Controller::Passenger