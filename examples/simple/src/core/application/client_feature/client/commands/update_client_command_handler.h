// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/commands/update_client_command.h"
#include "simple_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include "result.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Commands;

namespace Simple::Application::Features::Client::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_CLIENT_EXPORT UpdateClientCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateClientCommandHandler(InterfaceClientRepository *repository);
    Result<ClientDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateClientCommand &request);
    Result<ClientDTO> restore();

  Q_SIGNALS:
    void clientUpdated(Simple::Contracts::DTO::Client::ClientDTO clientDto);
    void clientDetailsUpdated(int id);

  private:
    InterfaceClientRepository *m_repository;
    Result<ClientDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateClientCommand &request);
    Result<ClientDTO> restoreImpl();
    Result<ClientDTO> saveOldState();
    Result<ClientDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Client::Commands