// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_client_query_handler.h"
#include "repository/interface_client_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Simple::Application::Features::Client::Queries;

GetAllClientQueryHandler::GetAllClientQueryHandler(InterfaceClientRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<ClientDTO>> GetAllClientQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllClientQueryHandler::handle called";

    Result<QList<ClientDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<ClientDTO>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllClientQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<QList<ClientDTO>> GetAllClientQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllClientQueryHandler::handleImpl called";

    // do
    auto clientResult = m_repository->getAll();

    QLN_RETURN_IF_ERROR(QList<ClientDTO>, clientResult)

    // map
    QList<ClientDTO> dtoList;

    for (const Simple::Entities::Client &client : clientResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Entities::Client, ClientDTO>(client);
        dtoList.append(dto);
    }

    qDebug() << "GetAllClientQueryHandler::handleImpl done";

    return Result<QList<ClientDTO>>(dtoList);
}

bool GetAllClientQueryHandler::s_mappingRegistered = false;

void GetAllClientQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Entities::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
}