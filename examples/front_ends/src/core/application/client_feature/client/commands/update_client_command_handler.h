// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/commands/update_client_command.h"
#include "front_ends_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Client;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Client::Commands;

namespace FrontEnds::Application::Features::Client::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CLIENT_EXPORT UpdateClientCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateClientCommandHandler(InterfaceClientRepository *repository);
    Result<ClientDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateClientCommand &request);
    Result<ClientDTO> restore();

  signals:
    void clientUpdated(FrontEnds::Contracts::DTO::Client::ClientDTO clientDto);
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

} // namespace FrontEnds::Application::Features::Client::Commands