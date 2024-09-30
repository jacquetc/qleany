// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_with_details_dto.h"
#include "client/queries/get_client_query.h"
#include "front_ends_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Client;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Client::Queries;

namespace FrontEnds::Application::Features::Client::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CLIENT_EXPORT GetClientWithDetailsQueryHandler : public QObject
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

} // namespace FrontEnds::Application::Features::Client::Queries