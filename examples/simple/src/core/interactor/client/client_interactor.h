// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/client_with_details_dto.h"
#include "client/create_client_dto.h"
#include "client/update_client_dto.h"
#include "event_dispatcher.h"
#include "simple_example_interactor_export.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Simple::Contracts::DTO::Client;

namespace Simple::Interactor::Client
{

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT ClientInteractor : public QObject
{
    Q_OBJECT
  public:
    explicit ClientInteractor(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static ClientInteractor *instance();

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
    static QPointer<ClientInteractor> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    ClientInteractor() = delete;
    ClientInteractor(const ClientInteractor &) = delete;
    ClientInteractor &operator=(const ClientInteractor &) = delete;
};

} // namespace Simple::Interactor::Client