// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_client_with_details_query_handler.h"
#include "repository/interface_client_repository.h"
#include "tools/automapper.h"

using namespace Simple;
using namespace Simple::Application::Features::Client::Queries;

GetClientWithDetailsQueryHandler::GetClientWithDetailsQueryHandler(InterfaceClientRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientWithDetailsDTO> GetClientWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                      const GetClientQuery &query)
{
    Result<ClientWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetClientQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<ClientWithDetailsDTO> GetClientWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                          const GetClientQuery &query)
{
    qDebug() << "GetClientWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto clientResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(ClientWithDetailsDTO, clientResult)

    Simple::Entities::Client client = clientResult.value();

    // map
    auto clientWithDetailsDTO = Simple::Tools::AutoMapper::map<Simple::Entities::Client, ClientWithDetailsDTO>(client);

    qDebug() << "GetClientWithDetailsQueryHandler::handleImpl done";

    return Result<ClientWithDetailsDTO>(clientWithDetailsDTO);
}

bool GetClientWithDetailsQueryHandler::s_mappingRegistered = false;

void GetClientWithDetailsQueryHandler::registerMappings()
{
    Simple::Tools::AutoMapper::registerMapping<Simple::Entities::Client,
                                               Contracts::DTO::Client::ClientWithDetailsDTO>();
}