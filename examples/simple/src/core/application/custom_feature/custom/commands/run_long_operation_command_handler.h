// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_custom_export.h"

#include "custom/commands/run_long_operation_command.h"

#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;

using namespace Simple::Contracts::CQRS::Custom::Commands;

namespace Simple::Application::Features::Custom::Commands
{
class SIMPLEEXAMPLE_APPLICATION_CUSTOM_EXPORT RunLongOperationCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RunLongOperationCommandHandler();

    Result<void> handle(QPromise<Result<void>> &progressPromise, const RunLongOperationCommand &request);

    Result<void> restore();

  signals:

    void runLongOperationChanged();

  private:
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const RunLongOperationCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Custom::Commands
