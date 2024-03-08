// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_current_time_query_handler.h"
#include <qleany/tools/automapper/automapper.h>

#include <QDebug>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Custom;

using namespace FrontEnds::Application::Features::Custom::Queries;

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

    // FrontEnds::Entities::Custom custom;

    // implement logic here
    // custom = Qleany::Tools::AutoMapper::AutoMapper::map<GetCurrentTimeReplyDTO,
    // FrontEnds::Entities::Custom>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    // auto customDTO = Qleany::Tools::AutoMapper::AutoMapper::map<FrontEnds::Entities::Custom,
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