// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_custom_export.h"
#include "custom/get_current_time_reply_dto.h"
#include "custom/queries/get_current_time_query.h"

#include "qleany/common/result.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Custom;

using namespace Simple::Contracts::CQRS::Custom::Queries;

namespace Simple::Application::Features::Custom::Queries
{
class SIMPLEEXAMPLE_APPLICATION_CUSTOM_EXPORT GetCurrentTimeQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetCurrentTimeQueryHandler();

    Result<GetCurrentTimeReplyDTO> handle(QPromise<Result<void>> &progressPromise, const GetCurrentTimeQuery &request);

  signals:
    void getCurrentTimeChanged(Simple::Contracts::DTO::Custom::GetCurrentTimeReplyDTO getCurrentTimeReplyDTO);

  private:
    Result<GetCurrentTimeReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                              const GetCurrentTimeQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Custom::Queries