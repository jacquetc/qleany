// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/client_with_details_dto.h"
#include "client/create_client_dto.h"
#include "client/update_client_dto.h"
#include "event_dispatcher.h"
#include "front_ends_example_controller_export.h"
#include "repository/interface_repository_provider.h"

#include "undo_redo/threaded_undo_redo_system.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>

using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Controller::UndoRedo;
using namespace FrontEnds::Contracts::DTO::Client;

namespace FrontEnds::Controller::Client
{

class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT ClientController : public QObject
{
    Q_OBJECT
public:
    explicit ClientController(InterfaceRepositoryProvider *repositoryProvider,
                              ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static ClientController *instance();

    Q_INVOKABLE QCoro::Task<ClientDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<ClientWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE QCoro::Task<QList<ClientDTO>> getAll() const;

    Q_INVOKABLE static Contracts::DTO::Client::CreateClientDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Client::UpdateClientDTO getUpdateDTO();

public Q_SLOTS:

    QCoro::Task<ClientDTO> create(const CreateClientDTO &dto);

    QCoro::Task<ClientDTO> update(const UpdateClientDTO &dto);

    QCoro::Task<bool> remove(int id);

private:
    static QPointer<ClientController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    ClientController() = delete;
    ClientController(const ClientController &) = delete;
    ClientController &operator=(const ClientController &) = delete;
};

} // namespace FrontEnds::Controller::Client