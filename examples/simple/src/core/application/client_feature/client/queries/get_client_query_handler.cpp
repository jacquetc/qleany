// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_client_query_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_client_repository.h"

using namespace Qleany;
using namespace Simple::Application::Features::Client::Queries;

GetClientQueryHandler::GetClientQueryHandler(InterfaceClientRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientDTO> GetClientQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetClientQuery &query)
{
    Result<ClientDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetClientQuery:" << ex.what();
    }
    return result;
}

Result<ClientDTO> GetClientQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const GetClientQuery &query)
{
    qDebug() << "GetClientQueryHandler::handleImpl called with id" << query.id;

    // do
    auto clientResult = m_repository->get(query.id);

    if (Q_UNLIKELY(clientResult.isError()))
    {
        return Result<ClientDTO>(clientResult.error());
    }

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(clientResult.value());

    qDebug() << "GetClientQueryHandler::handleImpl done";

    return Result<ClientDTO>(dto);
}

bool GetClientQueryHandler::s_mappingRegistered = false;

void GetClientQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
}