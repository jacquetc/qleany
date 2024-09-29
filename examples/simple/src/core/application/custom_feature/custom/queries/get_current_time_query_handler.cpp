// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_current_time_query_handler.h"
#include "tools/automapper.h"

#include <QDebug>

using namespace Simple;
using namespace Simple::Contracts::DTO::Custom;

using namespace Simple::Application::Features::Custom::Queries;

GetCurrentTimeQueryHandler::GetCurrentTimeQueryHandler()

{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<GetCurrentTimeReplyDTO> GetCurrentTimeQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                  const GetCurrentTimeQuery &request)
{
    Result<GetCurrentTimeReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<GetCurrentTimeReplyDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetCurrentTimeQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<GetCurrentTimeReplyDTO> GetCurrentTimeQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                      const GetCurrentTimeQuery &request)
{
    qDebug() << "GetCurrentTimeQueryHandler::handleImpl called";

    // Simple::Entities::Custom custom;

    // implement logic here
    // custom = Simple::Tools::AutoMapper::map<GetCurrentTimeReplyDTO, Simple::Entities::Custom>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    // auto customDTO = Simple::Tools::AutoMapper::map<Simple::Entities::Custom,
    // GetCurrentTimeReplyDTO>(customResult.value());
    //  dummy to compile
    GetCurrentTimeReplyDTO getCurrentTimeReplyDTO;

    // Q_EMIT signal
    Q_EMIT getCurrentTimeChanged(getCurrentTimeReplyDTO);

    // Return
    return Result<GetCurrentTimeReplyDTO>(getCurrentTimeReplyDTO);
}

bool GetCurrentTimeQueryHandler::s_mappingRegistered = false;

void GetCurrentTimeQueryHandler::registerMappings()
{
}