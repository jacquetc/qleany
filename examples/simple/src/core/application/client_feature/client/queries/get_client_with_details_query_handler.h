// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_with_details_dto.h"
#include "client/queries/get_client_query.h"
#include "simple_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Queries;

namespace Simple::Application::Features::Client::Queries
{
class SIMPLE_EXAMPLE_APPLICATION_CLIENT_EXPORT GetClientWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetClientWithDetailsQueryHandler(InterfaceClientRepository *repository);
    Result<ClientWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetClientQuery &query);

  private:
    InterfaceClientRepository *m_repository;
    Result<ClientWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetClientQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Client::Queries