// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "custom/get_current_time_reply_dto.h"
#include "custom/queries/get_current_time_query.h"
#include "front_ends_example_application_custom_export.h"

#include "result.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Custom;

using namespace FrontEnds::Contracts::CQRS::Custom::Queries;

namespace FrontEnds::Application::Features::Custom::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_CUSTOM_EXPORT GetCurrentTimeQueryHandler : public QObject
{
    Q_OBJECT
public:
    GetCurrentTimeQueryHandler();

    Result<GetCurrentTimeReplyDTO> handle(QPromise<Result<void>> &progressPromise, const GetCurrentTimeQuery &request);

Q_SIGNALS:
    void getCurrentTimeChanged(FrontEnds::Contracts::DTO::Custom::GetCurrentTimeReplyDTO getCurrentTimeReplyDTO);

private:
    Result<GetCurrentTimeReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetCurrentTimeQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Custom::Queries