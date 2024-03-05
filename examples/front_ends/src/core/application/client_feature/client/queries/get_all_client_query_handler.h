// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "front_ends_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Client;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Application::Features::Client::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CLIENT_EXPORT GetAllClientQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllClientQueryHandler(InterfaceClientRepository *repository);
    Result<QList<ClientDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceClientRepository *m_repository;
    Result<QList<ClientDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Client::Queries