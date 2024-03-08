// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/commands/create_client_command.h"
#include "repository/interface_client_repository.h"
#include "simple_example_application_client_export.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Commands;

namespace Simple::Application::Features::Client::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_CLIENT_EXPORT CreateClientCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateClientCommandHandler(InterfaceClientRepository *repository);

    Result<ClientDTO> handle(QPromise<Result<void>> &progressPromise, const CreateClientCommand &request);
    Result<ClientDTO> restore();

  Q_SIGNALS:
    void clientCreated(Simple::Contracts::DTO::Client::ClientDTO clientDto);
    void clientRemoved(int id);

  private:
    InterfaceClientRepository *m_repository;
    Result<ClientDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateClientCommand &request);
    Result<ClientDTO> restoreImpl();
    Result<Simple::Entities::Client> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Simple::Application::Features::Client::Commands