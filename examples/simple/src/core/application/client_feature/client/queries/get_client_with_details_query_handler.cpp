// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_client_with_details_query_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_client_repository.h"

using namespace Qleany;
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
        result = Result<ClientWithDetailsDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetClientQuery:" << ex.what();
    }
    return result;
}

Result<ClientWithDetailsDTO> GetClientWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                          const GetClientQuery &query)
{
    qDebug() << "GetClientWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto clientResult = m_repository->getWithDetails(query.id);

    if (Q_UNLIKELY(clientResult.isError()))
    {
        return Result<ClientWithDetailsDTO>(clientResult.error());
    }

    Simple::Domain::Client client = clientResult.value();

    // map
    auto WithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientWithDetailsDTO>(client);

    qDebug() << "GetClientWithDetailsQueryHandler::handleImpl done";

    return Result<ClientWithDetailsDTO>(WithDetailsDTO);
}

bool GetClientWithDetailsQueryHandler::s_mappingRegistered = false;

void GetClientWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Client,
                                                           Contracts::DTO::Client::ClientWithDetailsDTO>();
}