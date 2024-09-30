// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "run_long_operation_command_handler.h"
#include "tools/automapper.h"

#include <QDebug>

using namespace FrontEnds;

using namespace FrontEnds::Application::Features::Custom::Commands;

RunLongOperationCommandHandler::RunLongOperationCommandHandler()

{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> RunLongOperationCommandHandler::handle(QPromise<Result<void>> &progressPromise, const RunLongOperationCommand &request)
{
    Result<void> result;

    try {
        result = handleImpl(progressPromise, request);
    } catch (const std::exception &ex) {
        result = Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RunLongOperationCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<void> RunLongOperationCommandHandler::restore()
{
    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> RunLongOperationCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise, const RunLongOperationCommand &request)
{
    qDebug() << "RunLongOperationCommandHandler::handleImpl called";

    // implement logic here which will not be repeated on restore
    // custom = FrontEnds::Tools::AutoMapper::map<void, FrontEnds::Entities::Custom>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    // Q_EMIT signal
    // Q_EMIT runLongOperationChanged();

    // Return
    return Result<void>();
}

bool RunLongOperationCommandHandler::s_mappingRegistered = false;

void RunLongOperationCommandHandler::registerMappings()
{
}