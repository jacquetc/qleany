// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "client/commands/create_client_command.h"
#include "front_ends_example_application_client_export.h"
#include "repository/interface_client_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Entities;
using namespace FrontEnds::Contracts::DTO::Client;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Client::Commands;

namespace FrontEnds::Application::Features::Client::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CLIENT_EXPORT CreateClientCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateClientCommandHandler(InterfaceClientRepository *repository);

    Result<ClientDTO> handle(QPromise<Result<void>> &progressPromise, const CreateClientCommand &request);
    Result<ClientDTO> restore();

  Q_SIGNALS:
    void clientCreated(FrontEnds::Contracts::DTO::Client::ClientDTO clientDto);
    void clientRemoved(int id);

  private:
    InterfaceClientRepository *m_repository;
    Result<ClientDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateClientCommand &request);
    Result<ClientDTO> restoreImpl();
    Result<FrontEnds::Entities::Client> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace FrontEnds::Application::Features::Client::Commands