// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_client_query_handler.h"
#include "repository/interface_client_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Application::Features::Client::Queries;

GetClientQueryHandler::GetClientQueryHandler(InterfaceClientRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientDTO> GetClientQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetClientQuery &query)
{
    Result<ClientDTO> result;

    try {
        result = handleImpl(progressPromise, query);
    } catch (const std::exception &ex) {
        result = Result<ClientDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetClientQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<ClientDTO> GetClientQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetClientQuery &query)
{
    qDebug() << "GetClientQueryHandler::handleImpl called with id" << query.id;

    // do
    auto clientResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(ClientDTO, clientResult)

    // map
    auto dto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Client, ClientDTO>(clientResult.value());

    qDebug() << "GetClientQueryHandler::handleImpl done";

    return Result<ClientDTO>(dto);
}

bool GetClientQueryHandler::s_mappingRegistered = false;

void GetClientQueryHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Client, Contracts::DTO::Client::ClientDTO>(true, true);
}