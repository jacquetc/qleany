// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_current_time_query_handler.h"
#include "qleany/tools/automapper/automapper.h"

#include <QDebug>

using namespace Qleany;
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
        result = Result<GetCurrentTimeReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetCurrentTimeQuery:" << ex.what();
    }
    return result;
}

Result<GetCurrentTimeReplyDTO> GetCurrentTimeQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                      const GetCurrentTimeQuery &request)
{
    qDebug() << "GetCurrentTimeQueryHandler::handleImpl called";

    GetCurrentTimeReplyDTO getCurrentTimeReplyDTO;
    getCurrentTimeReplyDTO.setCurrentDateTime(QDateTime::currentDateTime());

    // emit signal
    emit getCurrentTimeChanged(getCurrentTimeReplyDTO);

    // Return
    return Result<GetCurrentTimeReplyDTO>(getCurrentTimeReplyDTO);
}

bool GetCurrentTimeQueryHandler::s_mappingRegistered = false;

void GetCurrentTimeQueryHandler::registerMappings()
{
}
