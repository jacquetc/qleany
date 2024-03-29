// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "event_dispatcher.h"
#include "front_ends_example_interactor_export.h"
#include "passenger/create_passenger_dto.h"
#include "passenger/passenger_dto.h"
#include "passenger/update_passenger_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace FrontEnds::Contracts::DTO::Passenger;

namespace FrontEnds::Interactor::Passenger
{

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT PassengerInteractor : public QObject
{
    Q_OBJECT
public:
    explicit PassengerInteractor(InterfaceRepositoryProvider *repositoryProvider,
                                 ThreadedUndoRedoSystem *undo_redo_system,
                                 QSharedPointer<EventDispatcher> eventDispatcher);

    static PassengerInteractor *instance();

    Q_INVOKABLE QCoro::Task<PassengerDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<QList<PassengerDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Passenger::CreatePassengerDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Passenger::UpdatePassengerDTO getUpdateDTO();

public Q_SLOTS:

    QCoro::Task<PassengerDTO> create(const CreatePassengerDTO &dto);

    QCoro::Task<PassengerDTO> update(const UpdatePassengerDTO &dto);

    QCoro::Task<bool> remove(int id);

private:
    static QPointer<PassengerInteractor> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    PassengerInteractor() = delete;
    PassengerInteractor(const PassengerInteractor &) = delete;
    PassengerInteractor &operator=(const PassengerInteractor &) = delete;
};

} // namespace FrontEnds::Interactor::Passenger